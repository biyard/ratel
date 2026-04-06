import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_efs as efs,
  aws_iam as iam,
  aws_logs as logs,
  aws_servicediscovery as sd,
  aws_elasticloadbalancingv2 as elbv2,
  RemovalPolicy,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export interface QdrantStackProps extends StackProps {
  stage: string;
  cluster: ecs.ICluster;
  vpc: ec2.IVpc;
  namespace: sd.PrivateDnsNamespace;
  // Shared SG from VpcServiceStack — every VPC service attaches this so
  // intra-SG traffic (including 6333/6334) is allowed by self-reference.
  sharedSecurityGroup: ec2.ISecurityGroup;
}

export class QdrantStack extends Stack {
  constructor(scope: Construct, id: string, props: QdrantStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { cluster, vpc, sharedSecurityGroup } = props;

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

    const container = taskDefinition.addContainer("QdrantContainer", {
      image: ecs.ContainerImage.fromRegistry("qdrant/qdrant:latest"),
      logging: new ecs.AwsLogDriver({
        streamPrefix: `ratel-${props.stage}-qdrant`,
        logRetention: logs.RetentionDays.TWO_WEEKS,
      }),
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

    // Allow task to connect to EFS (via shared SG — every VPC service can
    // mount, which is acceptable since EFS access is also gated by IAM).
    fileSystem.connections.allowDefaultPortFrom(sharedSecurityGroup);

    const fargateService = new ecs.FargateService(this, "QdrantService", {
      cluster,
      taskDefinition,
      desiredCount: 1,
      maxHealthyPercent: 100,
      minHealthyPercent: 0,
      assignPublicIp: true,
      vpcSubnets: { subnetType: ec2.SubnetType.PUBLIC },
      securityGroups: [sharedSecurityGroup],
      cloudMapOptions: {
        name: "qdrant",
        cloudMapNamespace: props.namespace,
        dnsRecordType: sd.DnsRecordType.A,
      },
    });

    // // --- Register with shared ALB for external gRPC access (port 6334) ---

    // props.albListener.addTargets(`QdrantTarget-${props.stage}`, {
    //   port: 6334,
    //   protocol: elbv2.ApplicationProtocol.HTTP,
    //   targets: [
    //     fargateService.loadBalancerTarget({
    //       containerName: "QdrantContainer",
    //       containerPort: 6334,
    //     }),
    //   ],
    //   conditions: [elbv2.ListenerCondition.hostHeaders([qdrantDomain])],
    //   priority: props.stage === "prod" ? 10 : 20,
    //   healthCheck: {
    //     path: "/healthz",
    //     port: "6333",
    //     protocol: elbv2.Protocol.HTTP,
    //   },
    // });

    // // --- Register with shared ALB for Qdrant UI/REST access (port 6333) ---

    // props.albListener.addTargets(`QdrantUiTarget-${props.stage}`, {
    //   port: 6333,
    //   protocol: elbv2.ApplicationProtocol.HTTP,
    //   targets: [
    //     fargateService.loadBalancerTarget({
    //       containerName: "QdrantContainer",
    //       containerPort: 6333,
    //     }),
    //   ],
    //   conditions: [elbv2.ListenerCondition.hostHeaders([qdrantUiDomain])],
    //   priority: props.stage === "prod" ? 11 : 21,
    //   healthCheck: {
    //     path: "/healthz",
    //     port: "6333",
    //     protocol: elbv2.Protocol.HTTP,
    //   },
    // });
  }
}
