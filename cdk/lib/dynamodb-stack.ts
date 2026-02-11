import * as cdk from "aws-cdk-lib";
import { Stack, StackProps, RemovalPolicy } from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";

export class GlobalTableStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const env = process.env.ENV;

    const table = new dynamodb.Table(this, "GlobalTable", {
      tableName: `ratel-${env}-main`,
      partitionKey: { name: "pk", type: dynamodb.AttributeType.STRING },
      sortKey: { name: "sk", type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PAY_PER_REQUEST,
      pointInTimeRecovery: true,
      stream: dynamodb.StreamViewType.NEW_AND_OLD_IMAGES,
      removalPolicy: RemovalPolicy.RETAIN,
      replicationRegions: ["eu-central-1", "us-east-1"],
      deletionProtection: true,
    });

    new cdk.CfnOutput(this, "MainTableStreamArn", {
      value: table.tableStreamArn ?? "",
      exportName: `ratel-${env}-main-stream-arn`,
    });

    new cdk.CfnOutput(this, "MainTableName", {
      value: table.tableName,
      exportName: `ratel-${env}-main-table-name`,
    });
  }
}
