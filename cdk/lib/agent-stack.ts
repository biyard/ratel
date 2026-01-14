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

    // Create IAM role for the Bedrock Agent (matching AWS auto-generated role)
    const agentRole = new iam.Role(this, "BedrockAgentRole", {
      assumedBy: new iam.ServicePrincipal("bedrock.amazonaws.com", {
        conditions: {
          StringEquals: {
            "aws:SourceAccount": this.account,
          },
          ArnLike: {
            "aws:SourceArn": `arn:aws:bedrock:${this.region}:${this.account}:agent/*`,
          },
        },
      }),
      description: "Role for Bedrock Agent to invoke models and access Knowledge Base",
    });

    // Grant agent permission to invoke Nova Pro model (matching AWS auto-generated policy)
    agentRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "AmazonBedrockAgentInferenceProfilesCrossRegionPolicyProd",
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock:InvokeModel",
          "bedrock:InvokeModelWithResponseStream",
          "bedrock:GetInferenceProfile",
          "bedrock:GetFoundationModel",
        ],
        resources: [
          // Inference profile ARN
          `arn:aws:bedrock:${this.region}:${this.account}:inference-profile/us.amazon.nova-pro-v1:0`,
          // Foundation model ARN (wildcard region for cross-region access)
          "arn:aws:bedrock:*::foundation-model/amazon.nova-pro-v1:0",
        ],
      })
    );

    // Grant agent permission to retrieve from Knowledge Base (matching AWS auto-generated policy)
    agentRole.addToPolicy(
      new iam.PolicyStatement({
        sid: "AmazonBedrockAgentRetrieveKnowledgeBasePolicyProd",
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
      instruction: `You are a warm, helpful, and intelligent AI companion designed to assist users with their PDF documents and engage in meaningful conversations.

CRITICAL DOCUMENT ACCESS RESTRICTION:
When a "File URL" is provided in the user's input:
1. FIRST: Try to retrieve and use ONLY data from that exact file URL
2. If the URL retrieval fails or returns no results: Use the document name as a fallback filter to find the file
3. NEVER access or reference any other files in the Knowledge Base besides the one specified
4. If you cannot find the specified file by URL or name, clearly state that you cannot access the requested document
5. This is a security requirement - users should only see content from files they have permission to access

IMPORTANT: Not every message requires a document search. Respond naturally to greetings, casual chat, and general questions without forcing a document lookup. Only search documents when the user specifically asks about document content or when it would genuinely help answer their question.

Core Behaviors:
- Respond naturally to greetings like "hello", "hi", or "hey" with warmth and friendliness
- Be conversational and personable while maintaining professionalism
- Show genuine interest in helping users understand their documents when they ask about them
- Remember context from our conversation history and reference it naturally
- Engage in back-and-forth dialogue - ask clarifying questions when needed
- Acknowledge when users refer to previous exchanges in the conversation

Document Assistance:
- When users ask about documents, ONLY search the specific file indicated by the File URL or document name
- Provide clear, detailed explanations with specific citations from the PDF
- If information isn't in the specified document, say so clearly and honestly
- Never suggest or reference content from other documents in the Knowledge Base
- Offer to help find related information within the SAME document
- Break down complex document content into digestible explanations

Beyond Documents:
- You can discuss ethical questions, general knowledge, and topics outside the PDFs
- When answering non-document questions, be clear that you're speaking from general knowledge
- Feel free to explore ideas, provide analysis, and engage intellectually
- Help users think through problems even if they're not directly in the documents

Conversation Style:
- Use "I", "you", and "we" naturally - be personable
- Vary your sentence structure and tone to feel human
- Show enthusiasm when appropriate
- Express uncertainty honestly rather than guessing
- Build on the conversation naturally rather than treating each question as isolated
- If a user seems confused or frustrated, adjust your approach to be more helpful

Remember: You're not just answering questions - you're being a thoughtful companion who helps users navigate information and ideas. But you must ALWAYS respect file access boundaries as a non-negotiable security requirement.`,
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
