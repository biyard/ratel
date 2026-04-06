import {
  aws_ec2 as ec2,
  aws_elasticloadbalancingv2 as elbv2,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  Stack,
  StackProps,
} from "aws-cdk-lib";
import * as route53Targets from "aws-cdk-lib/aws-route53-targets";
import { Construct } from "constructs";

export interface AlbStackProps extends StackProps {
  baseDomain: string;
  devDomain: string;
  prodDomain: string;
}

export class AlbStack extends Stack {
  public readonly listener: elbv2.ApplicationListener;
  public readonly albSecurityGroup: ec2.SecurityGroup;

  constructor(scope: Construct, id: string, props: AlbStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const vpc = ec2.Vpc.fromLookup(this, "Vpc", { isDefault: true });

    this.albSecurityGroup = new ec2.SecurityGroup(this, "AlbSG", {
      vpc,
      description: "Shared ALB security group",
      allowAllOutbound: true,
    });
    this.albSecurityGroup.addIngressRule(
      ec2.Peer.anyIpv4(),
      ec2.Port.tcp(443),
      "HTTPS from internet",
    );

    const alb = new elbv2.ApplicationLoadBalancer(this, "RatelAlb", {
      vpc,
      internetFacing: true,
      securityGroup: this.albSecurityGroup,
    });

    const zone = route53.HostedZone.fromLookup(this, "Zone", {
      domainName: props.baseDomain,
    });

    const cert = new acm.Certificate(this, "Cert", {
      domainName: `*.${props.prodDomain}`,
      subjectAlternativeNames: [`*.${props.devDomain}`],
      validation: acm.CertificateValidation.fromDns(zone),
    });

    this.listener = alb.addListener("HttpsListener", {
      port: 443,
      protocol: elbv2.ApplicationProtocol.HTTPS,
      certificates: [cert],
      defaultAction: elbv2.ListenerAction.fixedResponse(404, {
        contentType: "text/plain",
        messageBody: "Not Found",
      }),
    });

    for (const domain of ["qdrant", "qdrant-ui"]) {
      // DNS records for both domains pointing to the shared ALB
      new route53.ARecord(this, `Dev-${domain}-Dns`, {
        zone,
        recordName: `${domain}.${props.devDomain}`,
        target: route53.RecordTarget.fromAlias(
          new route53Targets.LoadBalancerTarget(alb),
        ),
      });

      new route53.ARecord(this, `Prod-${domain}-Dns`, {
        zone,
        recordName: `${domain}.${props.prodDomain}`,
        target: route53.RecordTarget.fromAlias(
          new route53Targets.LoadBalancerTarget(alb),
        ),
      });
    }
  }
}
