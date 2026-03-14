import {
  aws_ec2 as ec2,
  aws_ecs as ecs,
  Stack,
  aws_servicediscovery as sd,
  StackProps,
} from "aws-cdk-lib";

import { Construct } from "constructs";

export interface EcsClusterStackProps extends StackProps {
  stage: string;
}

export class EcsClusterStack extends Stack {
  public readonly cluster: ecs.Cluster;
  public readonly vpc: ec2.IVpc;
  public readonly namespace: sd.PrivateDnsNamespace;

  constructor(scope: Construct, id: string, props: EcsClusterStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    this.vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });
    this.cluster = new ecs.Cluster(this, "Cluster", { vpc: this.vpc });

    this.namespace = new sd.PrivateDnsNamespace(this, "Namespace", {
      name: `ratel-svc.local`,
      vpc: this.vpc,
    });

    this.namespace = new sd.PrivateDnsNamespace(this, `${props.stage}-Namespace`, {
      name: `ratel-${props.stage}-svc.local`,
      vpc: this.vpc,
    });
  }
}
