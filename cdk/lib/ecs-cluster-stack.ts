import { aws_ec2 as ec2, aws_ecs as ecs, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

export class EcsClusterStack extends Stack {
  public readonly cluster: ecs.Cluster;
  public readonly vpc: ec2.IVpc;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    this.vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });
    this.cluster = new ecs.Cluster(this, "Cluster", { vpc: this.vpc });
  }
}
