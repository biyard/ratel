import * as cdk from "aws-cdk-lib";
import * as bedrock from "aws-cdk-lib/aws-bedrock";
import * as iam from "aws-cdk-lib/aws-iam";
import { Construct } from "constructs";

export interface BedrockAgentProps {
  /**
   * The name of the agent
   */
  readonly agentName: string;

  /**
   * The ARN of the Knowledge Base to attach
   */
  readonly knowledgeBaseArn: string;

  /**
   * The ID of the Knowledge Base to attach
   */
  readonly knowledgeBaseId: string;

  /**
   * The foundation model to use (e.g., "amazon.nova-pro-v1:0")
   */
  readonly foundationModel: string;

  /**
   * The instruction/prompt for the agent
   */
  readonly instruction: string;

  /**
   * Description of the agent
   */
  readonly description?: string;

  /**
   * Idle session TTL in seconds
   * @default 600
   */
  readonly idleSessionTtlInSeconds?: number;

  /**
   * Memory configuration for the agent
   */
  readonly memoryConfiguration?: {
    enabledMemoryTypes: string[];
    storageDays: number;
    sessionSummaryConfiguration?: {
      maxRecentSessions: number;
    };
  };

  /**
   * The name of the agent alias
   * @default "production"
   */
  readonly aliasName?: string;
}

/**
 * Creates a Bedrock Agent with an associated alias and IAM role.
 */
export class BedrockAgent extends Construct {
  /**
   * The Bedrock Agent resource
   */
  public readonly agent: bedrock.CfnAgent;

  /**
   * The Bedrock Agent Alias resource
   */
  public readonly agentAlias: bedrock.CfnAgentAlias;

  /**
   * The IAM role for the agent
   */
  public readonly role: iam.Role;

  /**
   * The Agent ID
   */
  public readonly agentId: string;

  /**
   * The Agent ARN
   */
  public readonly agentArn: string;

  /**
   * The Agent Alias ID
   */
  public readonly agentAliasId: string;

  /**
   * The Agent Alias ARN
   */
  public readonly agentAliasArn: string;

  constructor(scope: Construct, id: string, props: BedrockAgentProps) {
    super(scope, id);

    const stack = cdk.Stack.of(this);

    // Create IAM role for the agent
    this.role = new iam.Role(this, "AgentRole", {
      assumedBy: new iam.ServicePrincipal("bedrock.amazonaws.com", {
        conditions: {
          StringEquals: {
            "aws:SourceAccount": stack.account,
          },
          ArnLike: {
            "aws:SourceArn": [
              `arn:${stack.partition}:bedrock:${stack.region}:${stack.account}:agent/*`,
              `arn:${stack.partition}:bedrock:${stack.region}:${stack.account}:agent-alias/*`,
            ],
          },
        },
      }),
      description: `Role for Bedrock Agent ${props.agentName}`,
    });

    // Grant agent permission to invoke the foundation model via inference profile
    const inferenceProfile = `apac.${props.foundationModel}`;

    const modelResources = [
      // Inference profile ARN (for cross-region models)
      `arn:aws:bedrock:${stack.region}:${stack.account}:inference-profile/${inferenceProfile}`,
      
      // Foundation model ARN
      `arn:aws:bedrock:${stack.region}::foundation-model/${props.foundationModel}`,
    ];

    this.role.addToPolicy(
      new iam.PolicyStatement({
        sid: "BedrockAgentInvokeModel",
        effect: iam.Effect.ALLOW,
        actions: [
          "bedrock:InvokeModel",
          "bedrock:InvokeModelWithResponseStream",
          "bedrock:GetInferenceProfile",
          "bedrock:GetFoundationModel",
        ],
        resources: modelResources,
      }),
    );

    // Grant agent permission to retrieve from Knowledge Base
    this.role.addToPolicy(
      new iam.PolicyStatement({
        sid: "BedrockAgentRetrieveKnowledgeBase",
        effect: iam.Effect.ALLOW,
        actions: ["bedrock:Retrieve"],
        resources: [props.knowledgeBaseArn],
      }),
    );

    // Create the Bedrock Agent
    this.agent = new bedrock.CfnAgent(this, "Agent", {
      agentName: props.agentName,
      agentResourceRoleArn: this.role.roleArn,
      foundationModel: inferenceProfile,
      description: props.description,
      instruction: props.instruction,
      idleSessionTtlInSeconds: props.idleSessionTtlInSeconds ?? 600,
      memoryConfiguration: props.memoryConfiguration
        ? {
            enabledMemoryTypes: props.memoryConfiguration.enabledMemoryTypes,
            storageDays: props.memoryConfiguration.storageDays,
            sessionSummaryConfiguration:
              props.memoryConfiguration.sessionSummaryConfiguration,
          }
        : undefined,
      knowledgeBases: [
        {
          knowledgeBaseId: props.knowledgeBaseId,
          description: "Knowledge base for the agent",
          knowledgeBaseState: "ENABLED",
        },
      ],
    });

    // Create agent alias
    this.agentAlias = new bedrock.CfnAgentAlias(this, "AgentAlias", {
      agentId: this.agent.attrAgentId,
      agentAliasName: props.aliasName ?? "development",
      description: `Alias for ${props.agentName}`,
    });

    this.agentAlias.addDependency(this.agent);

    // Export attributes
    this.agentId = this.agent.attrAgentId;
    this.agentArn = this.agent.attrAgentArn;
    this.agentAliasId = this.agentAlias.attrAgentAliasId;
    this.agentAliasArn = this.agentAlias.attrAgentAliasArn;
  }
}
