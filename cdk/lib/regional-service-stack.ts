import {
  Duration,
  Stack,
  StackProps,
  aws_ec2 as ec2,
  aws_ecs as ecs,
  aws_ecs_patterns as ecs_patterns,
  aws_elasticloadbalancingv2 as elbv2,
  aws_route53 as route53,
  aws_certificatemanager as acm,
} from "aws-cdk-lib";
import { Repository } from "aws-cdk-lib/aws-ecr";
import { Construct } from "constructs";

export interface RegionalServiceStackProps extends StackProps {
  // Domain parts, e.g. "api2.dev.ratel.foundation"
  fullDomainName: string;
  // Health check path for ALB target group
  healthCheckPath?: string;
  // Optional custom container image (default: nginx)
  imageUri?: string;
  commit: string;
}

export class RegionalServiceStack extends Stack {
  public readonly alb: elbv2.ApplicationLoadBalancer;

  constructor(scope: Construct, id: string, props: RegionalServiceStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const domain = props.fullDomainName;
    const healthPath = props.healthCheckPath ?? "/version";

    // 1) VPC across 2+ AZs
    const vpc = new ec2.Vpc(this, "Vpc", { maxAzs: 2, natGateways: 1 });

    // 2) ECS Cluster
    const cluster = new ecs.Cluster(this, "Cluster", { vpc });

    // 3) Route53 Hosted Zone lookup for "ratel.foundation"
    //    (Change if your zone name differs)
    const rootZone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: "ratel.foundation",
    });

    const cert = new acm.Certificate(this, "AlbCert", {
      domainName: domain,
      validation: acm.CertificateValidation.fromDns(rootZone),
    });
    // 4) Regional ACM certificate for the exact host (ALB terminates TLS)
    // const cert = new acm.DnsValidatedCertificate(this, "AlbCert", {
    //   domainName: domain,
    //   hostedZone: rootZone,
    //   region: Stack.of(this).region, // certificate must live in ALB's region
    // });

    // 5) ALB + Fargate service
    const repository = Repository.fromRepositoryName(
      this,
      "ApiRepository",
      "ratel/main-api",
    );

    const image = ecs.ContainerImage.fromEcrRepository(
      repository,
      props.commit,
    );

    const svc = new ecs_patterns.ApplicationLoadBalancedFargateService(
      this,
      "App",
      {
        cluster,
        cpu: 256,
        memoryLimitMiB: 512,
        desiredCount: 1,
        minHealthyPercent: 0,
        maxHealthyPercent: 200,
        publicLoadBalancer: true, // Internet-facing ALB
        protocol: elbv2.ApplicationProtocol.HTTPS,
        certificate: cert,
        redirectHTTP: true, // HTTP -> HTTPS
        listenerPort: 443,
        taskImageOptions: {
          image,
          containerPort: 3000,
        },
      },
    );

    // 6) Health checks for target group (match your app)
    svc.targetGroup.configureHealthCheck({
      path: healthPath,
      healthyHttpCodes: "200-399",
      interval: Duration.seconds(30),
      timeout: Duration.seconds(5),
      healthyThresholdCount: 2,
      unhealthyThresholdCount: 5,
    });

    this.alb = svc.loadBalancer;

    // Output ALB DNS (useful for debugging)
    this.exportValue(this.alb.loadBalancerDnsName, { name: `${id}-AlbDns` });
  }
}
