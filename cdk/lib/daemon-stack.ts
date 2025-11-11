import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_elasticloadbalancingv2 as elbv2,
  aws_iam as iam,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { Construct } from "constructs";

export interface DaemonStackProps extends StackProps {
  commit: string;
}

export class DaemonStack extends Stack {
  constructor(scope: Construct, id: string, props: DaemonStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });
    const cluster = new ecs.Cluster(this, "Cluster", { vpc });

    // 4) Task execution role
    const taskExecutionRole = new iam.Role(this, "TaskExecutionRole", {
      assumedBy: new iam.ServicePrincipal("ecs-tasks.amazonaws.com"),
    });

    taskExecutionRole.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        "service-role/AmazonECSTaskExecutionRolePolicy",
      ),
    );

    const fetcherContainerName = "FetcherContainer";
    const fetcherRepoName = "ratel/fetcher";

    const taskDefinition = new ecs.TaskDefinition(
      scope,
      "DaemonTaskDefinitionV2",
      {
        compatibility: ecs.Compatibility.FARGATE,
        cpu: "256",
        memoryMiB: "512",
        executionRole: taskExecutionRole,
      },
    );

    const fetcherRepository = Repository.fromRepositoryName(
      scope,
      "fetcherRepository",
      fetcherRepoName,
    );
    const fetcherContainer = taskDefinition.addContainer(fetcherContainerName, {
      image: ecs.ContainerImage.fromEcrRepository(
        fetcherRepository,
        props.commit,
      ),
      logging: new ecs.AwsLogDriver({
        streamPrefix: `ratel-${process.env.ENV}-fetcher`,
      }),
    });

    fetcherContainer.addPortMappings({
      containerPort: 4000,
      protocol: ecs.Protocol.TCP,
    });

    new ecs.FargateService(scope, "DaemonServiceV2", {
      cluster,
      taskDefinition,
      desiredCount: 1,
      maxHealthyPercent: 100,
      minHealthyPercent: 0,
      assignPublicIp: true,
    });
  }
}
