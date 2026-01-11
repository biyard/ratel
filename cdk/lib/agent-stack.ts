import {
  Stack,
  StackProps,
  aws_iam as iam,
  aws_bedrock as bedrock,
  CfnOutput,
} from "aws-cdk-lib";
import { Construct } from "constructs";

export interface AgentStackProps extends StackProps {
  stage: string;
  knowledgeBaseId: string;
  knowledgeBaseArn: string;
}

export class AgentStack extends Stack {
  public readonly agent: bedrock.CfnAgent;
  public readonly agentAlias: bedrock.CfnAgentAlias;

  constructor(scope: Construct, id: string, props: AgentStackProps) {
    super(scope, id, props);

    const { stage, knowledgeBaseId, knowledgeBaseArn } = props;

    // Create IAM role for the Bedrock Agent
    const agentRole = new iam.Role(this, "BedrockAgentRole", {
      assumedBy: new iam.ServicePrincipal("bedrock.amazonaws.com"),
      description: "Role for Bedrock Agent to invoke models and access Knowledge Base",
    });

    // Grant agent permission to invoke Nova Pro model
    agentRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:InvokeModel"],
        resources: [
          `arn:aws:bedrock:${this.region}::foundation-model/us.amazon.nova-pro-v1:0`,
        ],
      })
    );

    // Grant agent permission to retrieve from Knowledge Base
    agentRole.addToPolicy(
      new iam.PolicyStatement({
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:Retrieve"],
        resources: [knowledgeBaseArn],
      })
    );

    // Create the Bedrock Agent
    this.agent = new bedrock.CfnAgent(this, "BedrockAgent", {
      agentName: `pdf-agent-${stage}`,
      agentResourceRoleArn: agentRole.roleArn,
      foundationModel: "us.amazon.nova-pro-v1:0",
      description: "Agent connected to PDF Knowledge Base with Nova Pro",
      instruction: `You are a helpful AI assistant with access to a knowledge base containing PDF documents. 
When answering questions, search the knowledge base first and provide accurate, detailed responses based on the documents.
Always cite the source documents when providing information from the knowledge base.`,
      idleSessionTtlInSeconds: 600, // 10 minutes idle timeout
      
      // Enable memory for sessions
      memoryConfiguration: {
        enabledMemoryTypes: ["SESSION_SUMMARY"],
        storageDays: 30, // 30 days expiry
        sessionSummaryConfiguration: {
          maxRecentSessions: 20, // Max 20 sessions
        },
      },

      // Connect to Knowledge Base
      knowledgeBases: [
        {
          knowledgeBaseId: knowledgeBaseId,
          description: "PDF documents knowledge base",
          knowledgeBaseState: "ENABLED",
        },
      ],
    });

    // Prepare the agent (creates a new version)
    // This is done automatically when creating an alias

    // Create agent alias for production use
    this.agentAlias = new bedrock.CfnAgentAlias(this, "BedrockAgentAlias", {
      agentId: this.agent.attrAgentId,
      agentAliasName: "production",
      description: "Production alias for the PDF agent",
    });

    // Add dependency
    this.agentAlias.addDependency(this.agent);

    // Outputs
    new CfnOutput(this, "AgentId", {
      value: this.agent.attrAgentId,
      description: "Bedrock Agent ID",
      exportName: `${stage}-agent-id`,
    });

    new CfnOutput(this, "AgentArn", {
      value: this.agent.attrAgentArn,
      description: "Bedrock Agent ARN",
      exportName: `${stage}-agent-arn`,
    });

    new CfnOutput(this, "AgentAliasId", {
      value: this.agentAlias.attrAgentAliasId,
      description: "Bedrock Agent Alias ID",
      exportName: `${stage}-agent-alias-id`,
    });

    new CfnOutput(this, "AgentAliasArn", {
      value: this.agentAlias.attrAgentAliasArn,
      description: "Bedrock Agent Alias ARN",
      exportName: `${stage}-agent-alias-arn`,
    });
  }
}
