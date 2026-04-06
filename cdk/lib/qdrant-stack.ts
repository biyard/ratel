import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_efs as efs,
  aws_iam as iam,
  aws_logs as logs,
  aws_servicediscovery as sd,
  aws_apigatewayv2 as apigw,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  RemovalPolicy,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import { HttpServiceDiscoveryIntegration } from "aws-cdk-lib/aws-apigatewayv2-integrations";

export interface QdrantStackProps extends StackProps {
  stage: string;
  cluster: ecs.ICluster;
  vpc: ec2.IVpc;
  qdrantApiKey?: string;
  baseDomain: string;
  vectorDomain: string;
  qdrantDomain: string;
  namespace: sd.PrivateDnsNamespace;
}

export class QdrantStack extends Stack {
  constructor(scope: Construct, id: string, props: QdrantStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const {
      cluster,
      vpc,
      qdrantApiKey,
      baseDomain,
      vectorDomain,
      qdrantDomain,
    } = props;

    // Security group for Qdrant
    const sg = new ec2.SecurityGroup(this, "QdrantSG", {
      vpc,
      description: "Qdrant vector DB security group",
      allowAllOutbound: true,
    });

    sg.addIngressRule(
      ec2.Peer.ipv4(vpc.vpcCidrBlock),
      ec2.Port.tcp(6333),
      "Qdrant REST API",
    );
    sg.addIngressRule(
      ec2.Peer.ipv4(vpc.vpcCidrBlock),
      ec2.Port.tcp(6334),
      "Qdrant gRPC",
    );

    // EFS for persistent storage (use public subnets since default VPC has no private subnets)
    const fileSystem = new efs.FileSystem(this, "QdrantEfs", {
      vpc,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
      performanceMode: efs.PerformanceMode.GENERAL_PURPOSE,
      throughputMode: efs.ThroughputMode.BURSTING,
      removalPolicy: RemovalPolicy.RETAIN,
      encrypted: false,
      enableAutomaticBackups: true,
    });

    const accessPoint = fileSystem.addAccessPoint("QdrantAccessPoint", {
      path: "/qdrant-data",
      createAcl: { ownerGid: "1000", ownerUid: "1000", permissions: "755" },
      posixUser: { gid: "1000", uid: "1000" },
    });

    // Task execution role
    const taskExecutionRole = new iam.Role(this, "TaskExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
    });
    taskExecutionRole.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AmazonECSTaskExecutionRolePolicy",
      ),
    );

    // Task definition - minimal Fargate spec
    const taskDefinition = new ecs.TaskDefinition(this, "QdrantTaskDef", {
      compatibility: ecs.Compatibility.FARGATE,
      cpu: "256",
      memoryMiB: "512",
      executionRole: taskExecutionRole,
    });

    // Mount EFS volume
    taskDefinition.addVolume({
      name: "qdrant-storage",
      efsVolumeConfiguration: {
        fileSystemId: fileSystem.fileSystemId,
        transitEncryption: "ENABLED",
        authorizationConfig: {
          accessPointId: accessPoint.accessPointId,
          iam: "ENABLED",
        },
      },
    });

    // Grant EFS access to task role
    fileSystem.grantRootAccess(taskDefinition.taskRole);

    // Container environment
    const containerEnv: Record<string, string> = {};
    if (qdrantApiKey) {
      containerEnv["QDRANT__SERVICE__API_KEY"] = qdrantApiKey;
    }

    const container = taskDefinition.addContainer("QdrantContainer", {
      image: ecs.ContainerImage.fromRegistry("qdrant/qdrant:latest"),
      logging: new ecs.AwsLogDriver({
        streamPrefix: `ratel-${props.stage}-qdrant`,
        logRetention: logs.RetentionDays.TWO_WEEKS,
      }),
      environment: containerEnv,
    });

    container.addPortMappings(
      { containerPort: 6333, protocol: ecs.Protocol.TCP },
      { containerPort: 6334, protocol: ecs.Protocol.TCP },
    );

    container.addMountPoints({
      sourceVolume: "qdrant-storage",
      containerPath: "/qdrant/storage",
      readOnly: false,
    });

    // Allow task to connect to EFS
    fileSystem.connections.allowDefaultPortFrom(sg);

    const fargateService = new ecs.FargateService(this, "QdrantService", {
      cluster,
      taskDefinition,
      desiredCount: 1,
      maxHealthyPercent: 100,
      minHealthyPercent: 0,
      assignPublicIp: true,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
      securityGroups: [sg],
      cloudMapOptions: {
        name: "vector",
        cloudMapNamespace: props.namespace,
        dnsRecordType: sd.DnsRecordType.SRV,
        container,
        containerPort: 6333,
      },
    });

    // --- API Gateway with custom domain vector.ratel.foundation ---

    // Filter subnets to exclude AZs where API Gateway VPC Link is not available (apne2-az4)
    const supportedSubnets = vpc.publicSubnets.filter(
      (s) => s.availabilityZone !== "ap-northeast-2d",
    );

    const vpcLink = new apigw.VpcLink(this, "QdrantVpcLink", {
      vpc,
      subnets: { subnets: supportedSubnets },
      securityGroups: [sg],
    });

    const httpApi = new apigw.HttpApi(this, "QdrantHttpApi", {
      apiName: `ratel-${props.stage}-qdrant-api`,
      description: "Qdrant Vector DB API Gateway",
    });

    const qdrantIntegration = new HttpServiceDiscoveryIntegration(
      "QdrantIntegration",
      fargateService.cloudMapService!,
      { vpcLink },
    );

    httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [apigw.HttpMethod.ANY],
      integration: qdrantIntegration,
    });

    httpApi.addRoutes({
      path: "/",
      methods: [apigw.HttpMethod.ANY],
      integration: qdrantIntegration,
    });

    // DNS + Certificate
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    const cert = new acm.Certificate(this, "QdrantCert", {
      domainName: vectorDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    const domainName = new apigw.DomainName(this, "QdrantDomain", {
      domainName: vectorDomain,
      certificate: cert,
    });

    new apigw.ApiMapping(this, "QdrantApiMapping", {
      api: httpApi,
      domainName,
    });

    new route53.ARecord(this, "QdrantAliasA", {
      zone,
      recordName: vectorDomain,
      target: route53.RecordTarget.fromAlias({
        bind: () => ({
          dnsName: domainName.regionalDomainName,
          hostedZoneId: domainName.regionalHostedZoneId,
        }),
      }),
    });

    // --- gRPC API Gateway with custom domain (port 6334) ---

    const grpcCloudMapService = new sd.Service(this, "QdrantGrpcService", {
      namespace: props.namespace,
      name: "qdrant",
      dnsRecordType: sd.DnsRecordType.SRV,
    });

    grpcCloudMapService.registerNonIpInstance("QdrantGrpcInstance", {
      customAttributes: {
        AWS_INSTANCE_IPV4: `vector.${props.namespace.namespaceName}`,
        AWS_INSTANCE_PORT: "6334",
      },
    });

    const grpcHttpApi = new apigw.HttpApi(this, "QdrantGrpcHttpApi", {
      apiName: `ratel-${props.stage}-qdrant-grpc-api`,
      description: "Qdrant Vector DB gRPC API Gateway",
    });

    const grpcIntegration = new HttpServiceDiscoveryIntegration(
      "QdrantGrpcIntegration",
      grpcCloudMapService,
      { vpcLink },
    );

    grpcHttpApi.addRoutes({
      path: "/{proxy+}",
      methods: [apigw.HttpMethod.ANY],
      integration: grpcIntegration,
    });

    grpcHttpApi.addRoutes({
      path: "/",
      methods: [apigw.HttpMethod.ANY],
      integration: grpcIntegration,
    });

    const grpcCert = new acm.Certificate(this, "QdrantGrpcCert", {
      domainName: qdrantDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    const grpcDomainName = new apigw.DomainName(this, "QdrantGrpcDomain", {
      domainName: qdrantDomain,
      certificate: grpcCert,
    });

    new apigw.ApiMapping(this, "QdrantGrpcApiMapping", {
      api: grpcHttpApi,
      domainName: grpcDomainName,
    });

    new route53.ARecord(this, "QdrantGrpcAliasA", {
      zone,
      recordName: qdrantDomain,
      target: route53.RecordTarget.fromAlias({
        bind: () => ({
          dnsName: grpcDomainName.regionalDomainName,
          hostedZoneId: grpcDomainName.regionalHostedZoneId,
        }),
      }),
    });
  }
}
