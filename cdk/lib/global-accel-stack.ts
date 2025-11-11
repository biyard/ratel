import {
  Stack,
  StackProps,
  aws_route53 as route53,
  aws_certificatemanager as acm,
} from "aws-cdk-lib";
import { Construct } from "constructs";
import * as cloudfront from "aws-cdk-lib/aws-cloudfront";
import * as origins from "aws-cdk-lib/aws-cloudfront-origins";
import * as cdk from "aws-cdk-lib";
import * as targets from "aws-cdk-lib/aws-route53-targets";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as s3deploy from "aws-cdk-lib/aws-s3-deployment";

export interface GlobalAccelStackProps extends StackProps {
  commit: string;
  stage: string;

  webDomain: string;
  apiDomain: string;
  baseDomain: string;
}

export class GlobalAccelStack extends Stack {
  constructor(scope: Construct, id: string, props: GlobalAccelStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { commit, stage, webDomain, apiDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    const cert = new acm.Certificate(this, "Cert", {
      domainName: webDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // API Gateway Custom Domain Origin
    const origin = new origins.HttpOrigin(apiDomain, {
      protocolPolicy: cloudfront.OriginProtocolPolicy.HTTPS_ONLY,
      originSslProtocols: [cloudfront.OriginSslPolicy.TLS_V1_2],
      readTimeout: cdk.Duration.seconds(30),
      keepaliveTimeout: cdk.Duration.seconds(5),
    });

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

    // CloudFront cert (must be in us-east-1). Use provided ARN or create DNS‑validated one.
    const cachedS3Prop = {
      origin: s3Origin,
      cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
      compress: true,
    };

    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin,
        cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED,
        // Use ALL_VIEWER_EXCEPT_HOST_HEADER for API Gateway
        // CloudFront will automatically set Host header to the origin domain (api.dev.ratel.foundation)
        originRequestPolicy:
          cloudfront.OriginRequestPolicy.ALL_VIEWER_EXCEPT_HOST_HEADER,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      additionalBehaviors: {
        "/metadata/*": cachedS3Prop,
        "/assets/*": cachedS3Prop,
        "/icons/*": cachedS3Prop,
        "/images/*": cachedS3Prop,
        "/public/*": cachedS3Prop,
        "/animations/*": cachedS3Prop,
        "/documents/*": cachedS3Prop,
        "/logos/*": cachedS3Prop,
        "/sounds/*": cachedS3Prop,
        "/videos/*": cachedS3Prop,
        "/*.js": cachedS3Prop,
        "/*.css": cachedS3Prop,
        "/*.html": cachedS3Prop,
        "/*.ico": cachedS3Prop,
        "/*.svg": cachedS3Prop,
        "/*.avif": cachedS3Prop,
        "/*.png": cachedS3Prop,
        "/*.wasm": cachedS3Prop,
      },

      domainNames: [webDomain],
      certificate: cert,
      httpVersion: cloudfront.HttpVersion.HTTP2_AND_3,
      priceClass: cloudfront.PriceClass.PRICE_CLASS_ALL,
    });

    // ---- Route53 alias for the end-user domain → CloudFront ----
    new route53.ARecord(this, "AliasV4", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""), // e.g., 'dev'
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(distribution),
      ),
    });
    new route53.AaaaRecord(this, "AliasV6", {
      zone,
      recordName: webDomain.replace(`.${baseDomain}`, ""),
      target: route53.RecordTarget.fromAlias(
        new targets.CloudFrontTarget(distribution),
      ),
    });

    // new s3deploy.BucketDeployment(this, `RatelWebBucketDeployment-${stage}`, {
    //   destinationBucket: staticBucket,
    //   distribution: distribution,
    //   distributionPaths: ["/*"],
    //   sources: [s3deploy.Source.asset("dist")],
    // });
  }
}
