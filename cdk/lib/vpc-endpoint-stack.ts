import { aws_ec2 as ec2, Stack, StackProps } from "aws-cdk-lib";
import { Construct } from "constructs";

export interface VpcEndpointStackProps extends StackProps {}

/**
 * Singleton VPC endpoint + shared SG stack. Because VPC endpoints can only
 * exist **once per VPC**, this stack is NOT stage-scoped — it is shared across
 * all stages (dev/prod) that target the same VPC and region.
 *
 * Owns:
 * - **Shared security group** (`sharedSecurityGroup`) — every service in the
 *   VPC attaches this. Members can reach each other on any port via SG
 *   self-reference, so Lambda ↔ Qdrant ↔ ECS traffic needs no per-pair rules.
 * - **DynamoDB gateway endpoint** — free, uses route tables (no ENIs).
 * - **Bedrock runtime interface endpoint** with a dedicated endpoint SG that
 *   only accepts 443 from the shared SG.
 *
 * Downstream stage stacks (`EcsClusterStack`, `QdrantStack`, etc.) import
 * `sharedSecurityGroup` and attach it to their services.
 */
export class VpcEndpointStack extends Stack {
  public readonly vpc: ec2.IVpc;
  public readonly sharedSecurityGroup: ec2.SecurityGroup;

  constructor(scope: Construct, id: string, props: VpcEndpointStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    this.vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });

    this.sharedSecurityGroup = new ec2.SecurityGroup(this, "SharedServiceSG", {
      vpc: this.vpc,
      description: "Shared SG for all ratel VPC services (cross-stage)",
      allowAllOutbound: true,
    });
    // Self-reference: members of this SG can talk to each other on any port.
    this.sharedSecurityGroup.addIngressRule(
      this.sharedSecurityGroup,
      ec2.Port.allTraffic(),
      "Intra-SG traffic between shared services",
    );

    // DynamoDB gateway endpoint — free, uses route tables (no ENIs).
    new ec2.GatewayVpcEndpoint(this, "DynamoDbEndpoint", {
      vpc: this.vpc,
      service: ec2.GatewayVpcEndpointAwsService.DYNAMODB,
    });

    // Bedrock interface endpoint — dedicated SG that only accepts 443 from
    // the shared SG. Private DNS lets the SDK use the default endpoint URL.
    const bedrockEndpointSg = new ec2.SecurityGroup(this, "BedrockEndpointSG", {
      vpc: this.vpc,
      description: "Bedrock runtime VPC interface endpoint",
      allowAllOutbound: false,
    });
    bedrockEndpointSg.addIngressRule(
      this.sharedSecurityGroup,
      ec2.Port.allTraffic(),
      "Shared services to Bedrock runtime",
    );

    new ec2.InterfaceVpcEndpoint(this, "BedrockRuntimeEndpoint", {
      vpc: this.vpc,
      service: ec2.InterfaceVpcEndpointAwsService.BEDROCK_RUNTIME,
      subnets: { subnetType: ec2.SubnetType.PUBLIC },
      privateDnsEnabled: true,
      securityGroups: [bedrockEndpointSg],
    });

    // SES interface endpoint — dedicated SG that only accepts 443 from
    // the shared SG. Private DNS lets the SDK use the default endpoint URL.
    const sesEndpointSg = new ec2.SecurityGroup(this, "SesEndpointSG", {
      vpc: this.vpc,
      description: "SES VPC interface endpoint",
      allowAllOutbound: false,
    });
    sesEndpointSg.addIngressRule(
      this.sharedSecurityGroup,
      ec2.Port.allTraffic(),
      "Shared services to SES",
    );

    new ec2.InterfaceVpcEndpoint(this, "SesEmailEndpoint", {
      vpc: this.vpc,
      service: ec2.InterfaceVpcEndpointAwsService.EMAIL,
      subnets: { subnetType: ec2.SubnetType.PUBLIC },
      privateDnsEnabled: true,
      securityGroups: [sesEndpointSg],
    });
  }
}
