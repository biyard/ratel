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

    const imageCachePolicy = `NextImageCachePolicy-${stage}`;

    // ALB Domain
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

    const nextImageCachePolicy = new cloudfront.CachePolicy(
      this,
      imageCachePolicy,
      {
        cachePolicyName: imageCachePolicy,
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
    const cachedNextProp = {
      origin,
      cachePolicy: nextImageCachePolicy,
      compress: true,
    };
    const cachedS3Prop = {
      origin: s3Origin,
      cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
      compress: true,
    };

    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin,
        cachePolicy: cloudfront.CachePolicy.CACHING_DISABLED, // API/SSR default; tune if you want caching
        originRequestPolicy: cloudfront.OriginRequestPolicy.ALL_VIEWER,
        allowedMethods: cloudfront.AllowedMethods.ALLOW_ALL,
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      additionalBehaviors: {
        "/_next/image*": cachedNextProp,
        "/_next/static/*": cachedNextProp,
        "/metadata/*": cachedNextProp,
        "/assets/*": cachedNextProp,
        "/*.js": cachedNextProp,
        "/*.css": cachedNextProp,
        "/*.html": cachedNextProp,
        "/*.ico": cachedNextProp,
        "/*.svg": cachedNextProp,
        "/*.avif": cachedNextProp,
        "/*.png": cachedNextProp,
        "/*.wasm": cachedNextProp,
        "/icons/*": cachedNextProp,
        "/images/*": cachedNextProp,
        "/public/*": cachedNextProp,
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

    new s3deploy.BucketDeployment(this, "NextStaticDeployStatic", {
      destinationBucket: staticBucket,
      distribution: distribution,
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
      distribution: distribution,
      distributionPaths: ["/*"],
      sources: [
        s3deploy.Source.asset("public", {
          assetHash: commit,
          assetHashType: cdk.AssetHashType.CUSTOM,
        }),
      ],
    });
  }
}
