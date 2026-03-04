import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_efs as efs,
  aws_iam as iam,
  aws_logs as logs,
  aws_servicediscovery as sd,
  RemovalPolicy,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export interface QdrantStackProps extends StackProps {
  stage: string;
  cluster: ecs.ICluster;
  vpc: ec2.IVpc;
  qdrantApiKey?: string;
}

export class QdrantStack extends Stack {
  public readonly cloudMapService: sd.IService;
  public readonly securityGroup: ec2.ISecurityGroup;

  constructor(scope: Construct, id: string, props: QdrantStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { cluster, vpc, qdrantApiKey } = props;

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

    this.securityGroup = sg;

    // EFS for persistent storage
    const fileSystem = new efs.FileSystem(this, "QdrantEfs", {
      vpc,
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
      cpu: "2048",
      memoryMiB: "8196",
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

    // Cloud Map private DNS namespace for VPC Link service discovery
    const namespace = new sd.PrivateDnsNamespace(this, "QdrantNamespace", {
      name: `ratel-${props.stage}.local`,
      vpc,
    });

    const fargateService = new ecs.FargateService(this, "QdrantService", {
      cluster,
      taskDefinition,
      desiredCount: 1,
      maxHealthyPercent: 100,
      minHealthyPercent: 0,
      assignPublicIp: true,
      securityGroups: [sg],
      cloudMapOptions: {
        name: "qdrant",
        cloudMapNamespace: namespace,
        dnsRecordType: sd.DnsRecordType.A,
        container,
        containerPort: 6333,
      },
    });

    this.cloudMapService = fargateService.cloudMapService!;
  }
}
