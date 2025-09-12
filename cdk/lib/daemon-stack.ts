import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_elasticloadbalancingv2 as elbv2,
  aws_iam as iam,
} from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { RegionalServiceStack } from "./regional-service-stack";

export interface DaemonStackProps {
  commit: string;
  vpc: ec2.IVpc;
  cluster: ecs.ICluster;
  listener: elbv2.ApplicationListener;
  taskExecutionRole: iam.IRole;
}

export class DaemonStack {
  constructor(scope: RegionalServiceStack, props: DaemonStackProps) {
    const { vpc, cluster, listener, taskExecutionRole } = props;
    const healthPath = "/version";
    const fetcherContainerName = "FetcherContainer";
    const fetcherRepoName = "ratel/fetcher";

    const taskDefinition = new ecs.TaskDefinition(
      scope,
      "DaemonTaskDefinition",
      {
        compatibility: ecs.Compatibility.FARGATE,
        cpu: "256",
        memoryMiB: "512",
        executionRole: taskExecutionRole,
      }
    );

    const fetcherRepository = Repository.fromRepositoryName(
      scope,
      "fetcherRepository",
      fetcherRepoName
    );
    const fetcherContainer = taskDefinition.addContainer(fetcherContainerName, {
      image: ecs.ContainerImage.fromEcrRepository(
        fetcherRepository,
        props.commit
      ),
      logging: new ecs.AwsLogDriver({
        streamPrefix: `ratel-${process.env.ENV}-fetcher`,
      }),
    });

    fetcherContainer.addPortMappings({
      containerPort: 4000,
      protocol: ecs.Protocol.TCP,
    });

    const fargate = new ecs.FargateService(scope, "DaemonService", {
      cluster,
      taskDefinition,
      desiredCount: 1,
      maxHealthyPercent: 100,
      minHealthyPercent: 0,
      assignPublicIp: true,
    });
  }
}
