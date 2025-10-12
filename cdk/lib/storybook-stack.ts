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

export interface StorybookStackProps extends StackProps {
  commit: string;

  webDomain: string;
  baseDomain: string;
}

export class StorybookStack extends Stack {
  constructor(scope: Construct, id: string, props: StorybookStackProps) {
    super(scope, id, { ...props, crossRegionReferences: true });

    const { commit, webDomain, baseDomain } = props;
    const zone = route53.HostedZone.fromLookup(this, "RootZone", {
      domainName: baseDomain,
    });

    const cert = new acm.Certificate(this, "Cert", {
      domainName: webDomain,
      validation: acm.CertificateValidation.fromDns(zone),
    });

    // 1) S3 for static assets
    const staticBucket = new s3.Bucket(this, "StaticBucket", {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
    });

    const oai = new cloudfront.OriginAccessIdentity(this, "OAI");
    staticBucket.grantRead(oai);

    const origin = origins.S3BucketOrigin.withOriginAccessIdentity(
      staticBucket,
      {
        originAccessIdentity: oai,
      },
    );

    const distribution = new cloudfront.Distribution(this, "Distribution", {
      defaultBehavior: {
        origin,
        cachePolicy: cloudfront.CachePolicy.CACHING_OPTIMIZED,
        viewerProtocolPolicy: cloudfront.ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
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

    new s3deploy.BucketDeployment(this, "PublicDeployStatic", {
      destinationBucket: staticBucket,
      distribution: distribution,
      distributionPaths: ["/*"],
      sources: [
        s3deploy.Source.asset("storybook-static", {
          assetHash: commit,
          assetHashType: cdk.AssetHashType.CUSTOM,
        }),
      ],
    });
  }
}
