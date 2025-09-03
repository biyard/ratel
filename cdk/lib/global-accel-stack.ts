import {
  Stack,
  StackProps,
  aws_route53 as route53,
  aws_certificatemanager as acm,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import * as elbv2 from "aws-cdk-lib/aws-elasticloadbalancingv2";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as cdk from "aws-cdk-lib";
import * as targets from "aws-cdk-lib/aws-route53-targets";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as cf from "aws-cdk-lib/aws-cloudfront";
import * as s3deploy from "aws-cdk-lib/aws-s3-deployment";
import * as iam from "aws-cdk-lib/aws-iam";

export interface GlobalAccelStackProps extends StackProps {
  // Three ALBs built in regional stacks:
  euAlb: elbv2.IApplicationLoadBalancer;
  usAlb: elbv2.IApplicationLoadBalancer;
  krAlb: elbv2.IApplicationLoadBalancer;

  // DNS host like "dev.ratel.foundation"
  fullDomainName: string;
  commit: string;
}

export class GlobalAccelStack extends Stack {
  public readonly distribution: cloudfront.Distribution;

  constructor(scope: Construct, id: string, props: GlobalAccelStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { euAlb, usAlb, krAlb, fullDomainName, commit } = props;

    const webDomain = fullDomainName;
    const apiDomain = `api.${fullDomainName}`;
    const albDomain = `alb.${fullDomainName}`;

    // Root hosted zone derived from fullDomainName (e.g., ratel.foundation)
    const baseDomain = "ratel.foundation";
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });
    const cert = new acm.Certificate(this, "AlbCert", {
      domainName: webDomain,
      subjectAlternativeNames: [apiDomain],
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // Single origin hostname resolved via Route 53 latency policy

    // ---- Latency-based A/AAAA records: origin.<host> → per‑region ALBs ----
    // Note: we set `region` to the AWS region of the ALB; setIdentifier must be unique per record.
    const albEntries: Array<{
      id: string;
      region: string;
      alb: elbv2.IApplicationLoadBalancer;
    }> = [
      { id: "eu", region: "eu-central-1", alb: euAlb },
      { id: "us", region: "us-east-1", alb: usAlb },
      { id: "kr", region: "ap-northeast-2", alb: krAlb },
    ];

    albEntries.forEach(({ id: rid, region, alb }) => {
      new route53.CfnRecordSet(this, `LatencyA-${rid}`, {
        hostedZoneId: zone.hostedZoneId,
        name: albDomain,
        type: "A",
        setIdentifier: `alb-${rid}`,
        region,
        aliasTarget: {
          dnsName: alb.loadBalancerDnsName,
          hostedZoneId: alb.loadBalancerCanonicalHostedZoneId,
          evaluateTargetHealth: false,
        },
      });

      new route53.CfnRecordSet(this, `LatencyAAAA-${rid}`, {
        hostedZoneId: zone.hostedZoneId,
        name: albDomain,
        type: "AAAA",
        setIdentifier: `alb6-${rid}`,
        region,
        aliasTarget: {
          dnsName: alb.loadBalancerDnsName,
          hostedZoneId: alb.loadBalancerCanonicalHostedZoneId,
          evaluateTargetHealth: false,
        },
      });
    });

    // ---- CloudFront Distribution using latency-routed origin ----
    const origin = new origins.HttpOrigin(albDomain, {
      protocolPolicy: cloudfront.OriginProtocolPolicy.HTTPS_ONLY, // keep ALB HTTPS; if ALB is HTTP, switch to HTTP_ONLY
      originSslProtocols: [cloudfront.OriginSslPolicy.TLS_V1_2],
      readTimeout: cdk.Duration.seconds(30),
      keepaliveTimeout: cdk.Duration.seconds(5),
    });

    const assetsBucket = s3.Bucket.fromBucketName(
      this,
      "DefaultS3Bucket",
      fullDomainName,
    );

    // 1) S3 for static assets
    const staticBucket = new s3.Bucket(this, "NextStaticBucket", {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const oai = new cloudfront.OriginAccessIdentity(this, "OAI");
    staticBucket.grantRead(oai);

    const s3Origin = origins.S3BucketOrigin.withOriginAccessIdentity(
      staticBucket,
      {
        originAccessIdentity: oai,
      },
    );

    const nextImageCachePolicy = new cloudfront.CachePolicy(
      this,
      "NextImageCachePolicy",
      {
        cachePolicyName: "NextImageCachePolicy",
        defaultTtl: cdk.Duration.days(1),
        minTtl: cdk.Duration.seconds(60),
        maxTtl: cdk.Duration.days(365),
        queryStringBehavior: cloudfront.CacheQueryStringBehavior.allowList(
          "url",
          "w",
          "q",
        ),
        headerBehavior: cloudfront.CacheHeaderBehavior.none(),
        cookieBehavior: cloudfront.CacheCookieBehavior.none(),
        enableAcceptEncodingBrotli: true,
        enableAcceptEncodingGzip: true,
      },
    );

    // CloudFront cert (must be in us-east-1). Use provided ARN or create DNS‑validated one.
    const s3OriginProp = {
      origin: s3Origin,
      cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
      compress: true,
    };

    const cachedNextProp = {
      origin,
      cachePolicy: nextImageCachePolicy,
      compress: true,
    };

    this.distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin,
        cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED, // API/SSR default; tune if you want caching
        originRequestPolicy: cloudfront.OriginRequestPolicy.ALL_VIEWER,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      additionalBehaviors: {
        "/_next/*": cachedNextProp,
        "/metadata/*": s3OriginProp,
        "/assets/*": s3OriginProp,
        "/*.js": s3OriginProp,
        "/*.css": s3OriginProp,
        "/*.html": s3OriginProp,
        "/*.ico": s3OriginProp,
        "/*.svg": s3OriginProp,
        "/*.avif": s3OriginProp,
        "/*.png": s3OriginProp,
        "/*.wasm": s3OriginProp,
        "/icons/*": s3OriginProp,
        "/images/*": s3OriginProp,
        "/public/*": s3OriginProp,
      },

      domainNames: [webDomain, apiDomain],
      certificate: cert,
      httpVersion: cloudfront.HttpVersion.HTTP2_AND_3,
      priceClass: cloudfront.PriceClass.PRICE_CLASS_ALL,
    });

    // ---- Route53 alias for the end-user domain → CloudFront ----
    new route53.ARecord(this, "AliasV4", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""), // e.g., 'dev'
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(this.distribution),
      ),
    });
    new route53.AaaaRecord(this, "AliasV6", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(this.distribution),
      ),
    });

    new route53.ARecord(this, "ApiAliasV4", {
      zone,
      recordName: apiDomain.replace(`.${baseDomain}`, ""), // e.g., 'dev'
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(this.distribution),
      ),
    });
    new route53.AaaaRecord(this, "ApiAliasV6", {
      zone,
      recordName: apiDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(this.distribution),
      ),
    });

    new s3deploy.BucketDeployment(this, "NextStaticDeployStatic", {
      destinationBucket: staticBucket,
      distribution: this.distribution,
      distributionPaths: ["/_next/static/*"],
      sources: [
        s3deploy.Source.asset(".next/static", {
          assetHash: commit,
          assetHashType: cdk.AssetHashType.CUSTOM,
        }),
      ],
      destinationKeyPrefix: "_next/static",
    });

    new s3deploy.BucketDeployment(this, "PublicDeployStatic", {
      destinationBucket: staticBucket,
      distribution: this.distribution,
      distributionPaths: ["/*"],
      sources: [
        s3deploy.Source.asset("public", {
          assetHash: commit,
          assetHashType: cdk.AssetHashType.CUSTOM,
        }),
      ],
    });

    new cdk.CfnOutput(this, "CloudFrontDomain", {
      value: this.distribution.distributionDomainName,
    });

    new cdk.CfnOutput(this, "CloudFrontID", {
      value: this.distribution.distributionId,
    });
    new cdk.CfnOutput(this, "CloudFrontArn", {
      value: this.distribution.distributionArn,
    });

    new cdk.CfnOutput(this, "OriginHostname", { value: albDomain });
  }
}
