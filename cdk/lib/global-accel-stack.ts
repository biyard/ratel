import { Stack, StackProps, aws_route53 as route53 } from "aws-cdk-lib";
import { Construct } from "constructs";
import * as ga from "aws-cdk-lib/aws-globalaccelerator";
import * as ga_endpoints from "aws-cdk-lib/aws-globalaccelerator-endpoints";
import * as route53_targets from "aws-cdk-lib/aws-route53-targets";
import * as elbv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";

export interface GlobalAccelStackProps extends StackProps {
  // Three ALBs built in regional stacks:
  euAlb: elbv2.IApplicationLoadBalancer;
  usAlb: elbv2.IApplicationLoadBalancer;
  krAlb: elbv2.IApplicationLoadBalancer;

  // DNS host like "api2.dev.ratel.foundation"
  fullDomainName: string;
}

export class GlobalAccelStack extends Stack {
  public readonly accelerator: ga.Accelerator;

  constructor(scope: Construct, id: string, props: GlobalAccelStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    // 1) Global Accelerator (allocates 2 anycast static IPs)
    const acc = new ga.Accelerator(this, "Accelerator", {
      acceleratorName: "api2-dev-ratel-foundation",
    });

    // 2) Listener(s): HTTPS only (443). Add 80 if you need plain HTTP pass-through.
    const listener = acc.addListener("Https", {
      portRanges: [{ fromPort: 443, toPort: 443 }],
    });

    // 3) Endpoint groups per Region (attach each regional ALB)
    listener.addEndpointGroup("EU", {
      // Frankfurt
      region: "eu-central-1",
      healthCheckPort: 443,
      healthCheckPath: "/version",
      healthCheckProtocol: ga.HealthCheckProtocol.HTTPS,
      endpoints: [
        new ga_endpoints.ApplicationLoadBalancerEndpoint(props.euAlb, {
          preserveClientIp: true,
          weight: 100,
        }),
      ],
    });

    listener.addEndpointGroup("US", {
      // N. Virginia
      region: "us-east-1",
      healthCheckPort: 443,
      healthCheckPath: "/version",
      healthCheckProtocol: ga.HealthCheckProtocol.HTTPS,
      endpoints: [
        new ga_endpoints.ApplicationLoadBalancerEndpoint(props.usAlb, {
          preserveClientIp: true,
          weight: 100,
        }),
      ],
    });

    listener.addEndpointGroup("KR", {
      // Seoul
      region: "ap-northeast-2",
      healthCheckPort: 443,
      healthCheckPath: "/version",
      healthCheckProtocol: ga.HealthCheckProtocol.HTTPS,
      endpoints: [
        new ga_endpoints.ApplicationLoadBalancerEndpoint(props.krAlb, {
          preserveClientIp: true,
          weight: 100,
        }),
      ],
    });

    this.accelerator = acc;

    // 4) Route53 record: api2.dev.ratel.foundation -> Global Accelerator
    //    A/AAAA Alias to GA (supports dual-stack)
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: "ratel.foundation",
    });

    const recordName = props.fullDomainName.replace(".ratel.foundation", "");

    new route53.ARecord(this, "A-GA", {
      zone,
      recordName, // "api2.dev"
      target: route53.RecordTarget.fromAlias(
        new route53_targets.GlobalAcceleratorTarget(this.accelerator),
      ),
    });

    new route53.AaaaRecord(this, "AAAA-GA", {
      zone,
      recordName,
      target: route53.RecordTarget.fromAlias(
        new route53_targets.GlobalAcceleratorTarget(this.accelerator),
      ),
    });

    // Nice to have: output accelerator DNS & IPs
    this.exportValue(this.accelerator.dnsName, { name: "AcceleratorDns" });
  }
}
