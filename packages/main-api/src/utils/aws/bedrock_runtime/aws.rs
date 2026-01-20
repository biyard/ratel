use std::{collections::HashMap, time::Duration};

use crate::config;
use aws_config::{Region, retry::RetryConfig, timeout::TimeoutConfig};

use aws_sdk_bedrockagentruntime::{
    Client as AgentClient,
    types::{
        FilterAttribute, KnowledgeBaseRetrievalConfiguration,
        KnowledgeBaseRetrieveAndGenerateConfiguration, RetrievalFilter,
        RetrieveAndGenerateConfiguration, RetrieveAndGenerateInput,
    },
};
use aws_sdk_bedrockruntime::{
    Client, Config,
    config::Credentials,
    types::{ContentBlock, ConversationRole, Message},
};

use crate::{Error, Result};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum BedrockModel {
    NovaLite,
    NovaMicro,
}
#[derive(Clone)]
pub struct BedrockClient {
    client: Client,
    agent_client: AgentClient,
    model_arns: HashMap<BedrockModel, String>,
    agent_id: String,
    agent_alias_id: String,
}

impl BedrockClient {
    pub fn new() -> Self {
        let conf = config::get();
        let timeout_config = TimeoutConfig::builder()
            .operation_attempt_timeout(Duration::from_secs(120)) // Increased for Knowledge Base queries
            .operation_timeout(Duration::from_secs(120))
            .build();

        let retry_config = RetryConfig::standard().with_max_attempts(3);
        let aws_config = Config::builder()
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(conf.aws.access_key_id)
                    .secret_access_key(conf.aws.secret_access_key)
                    .provider_name("ratel")
                    .build(),
            )
            .region(Region::new(conf.aws.region))
            .timeout_config(timeout_config.clone())
            .retry_config(retry_config.clone())
            .behavior_version_latest()
            .build();

        let client = Client::from_conf(aws_config);

        // Create separate config for agent client
        let agent_config = aws_sdk_bedrockagentruntime::Config::builder()
            .credentials_provider(
                Credentials::builder()
                    .access_key_id(conf.aws.access_key_id)
                    .secret_access_key(conf.aws.secret_access_key)
                    .provider_name("ratel")
                    .build(),
            )
            .region(Region::new(conf.aws.region))
            .timeout_config(timeout_config)
            .retry_config(retry_config)
            .behavior_version_latest()
            .build();
        let agent_client = AgentClient::from_conf(agent_config);

        let model_arns = vec![
            (
                BedrockModel::NovaLite,
                conf.bedrock.nova_lite_model_id.to_string(),
            ),
            (
                BedrockModel::NovaMicro,
                conf.bedrock.nova_micro_model_id.to_string(),
            ),
        ]
        .into_iter()
        .collect::<HashMap<BedrockModel, String>>();

        Self {
            client,
            agent_client,
            model_arns,
            agent_id: conf.bedrock.agent_id.to_string(),
            agent_alias_id: conf.bedrock.agent_alias_id.to_string(),
        }
    }

    pub async fn send_message(
        &self,
        model: BedrockModel,
        prompt: String,
        content: Option<Vec<ContentBlock>>,
    ) -> Result<String> {
        let model_id = match self.model_arns.get(&model) {
            Some(id) => id,
            None => {
                return Err(Error::AwsBedrockError("Invalid model".to_string()));
            }
        };

        let contents = if let Some(mut c) = content {
            c.insert(0, ContentBlock::Text(prompt));
            c
        } else {
            vec![ContentBlock::Text(prompt)]
        };

        let message = Message::builder()
            .role(ConversationRole::User)
            .set_content(Some(contents))
            .build()
            .map_err(|e| {
                tracing::error!("Error building Bedrock message: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?;

        let bedrock_response = self
            .client
            .converse()
            .model_id(model_id)
            .messages(message)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Error calling Bedrock Converse: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?;
        tracing::debug!("Bedrock response: {:?}", bedrock_response.usage);

        let contents = bedrock_response
            .output()
            .ok_or(Error::AwsBedrockError("Empty Bedrock response".to_string()))?
            .as_message()
            .map_err(|e| {
                tracing::error!("Error extracting message from Bedrock response: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?
            .content();

        let text = contents
            .first()
            .ok_or(Error::AwsBedrockError(
                "Empty Bedrock response content".to_string(),
            ))?
            .as_text()
            .map_err(|e| {
                tracing::error!("Error extracting text from Bedrock content: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?
            .to_string();

        Ok(text)
    }

    /// Invoke agent with session management
    pub async fn invoke_agent(
        &self,
        session_id: String,
        input_text: String,
    ) -> Result<(String, String)> {
        tracing::debug!(
            "bedlock agent id: {} bedlock agent alias id: {}",
            self.agent_id,
            self.agent_alias_id
        );
        let response = self
            .agent_client
            .invoke_agent()
            .agent_id(&self.agent_id)
            .agent_alias_id(&self.agent_alias_id)
            .session_id(&session_id)
            .input_text(input_text)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Error invoking Bedrock Agent: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?;

        let session_id = response.session_id().to_string();

        // Collect chunks from the event stream
        let mut completion_text = String::new();
        let mut event_stream = response.completion;

        while let Some(event) = event_stream.recv().await.map_err(|e| {
            tracing::error!("Error receiving agent event: {:?}", e);
            Error::AwsBedrockError(format!("{:?}", e))
        })? {
            if let Ok(chunk) = event.as_chunk() {
                if let Some(bytes) = chunk.bytes() {
                    let text = String::from_utf8_lossy(bytes.as_ref());
                    completion_text.push_str(&text);
                }
            }
        }

        if completion_text.is_empty() {
            return Err(Error::AwsBedrockError("Empty agent response".to_string()));
        }

        Ok((completion_text, session_id))
    }

    /// Retrieve and generate using Knowledge Base with session management
    pub async fn retrieve_and_generate(
        &self,
        knowledge_base_id: String,
        session_id: Option<String>,
        query: String,
        s3_uri_filter: Option<String>,
    ) -> Result<(String, String)> {
        let conf = config::get();
        let model_arn = format!(
            "arn:aws:bedrock:{}::foundation-model/amazon.nova-pro-v1:0",
            conf.aws.region
        );

        tracing::info!(
            "retrieve_and_generate called with KB ID: {}",
            knowledge_base_id
        );
        tracing::info!("Model ARN: {}", model_arn);
        tracing::info!("Query: {}", query);
        if let Some(ref uri) = s3_uri_filter {
            tracing::info!("S3 URI Filter: {}", uri);
        }

        // Build retrieval filter if S3 URI is provided
        let mut kb_config_builder = KnowledgeBaseRetrieveAndGenerateConfiguration::builder()
            .knowledge_base_id(knowledge_base_id)
            .model_arn(model_arn);

        if let Some(uri) = s3_uri_filter {
            // Create filter to match specific S3 URI
            let filter = RetrievalFilter::Equals(
                aws_sdk_bedrockagentruntime::types::FilterAttribute::builder()
                    .key("x-amz-bedrock-kb-data-source-uri")
                    .value(aws_smithy_types::Document::String(uri))
                    .build()
                    .map_err(|e| {
                        tracing::error!("Error building filter attribute: {:?}", e);
                        Error::AwsBedrockError(format!("{:?}", e))
                    })?,
            );

            let retrieval_config = KnowledgeBaseRetrievalConfiguration::builder()
                .vector_search_configuration(
                    aws_sdk_bedrockagentruntime::types::KnowledgeBaseVectorSearchConfiguration::builder()
                        .filter(filter)
                        .build()
                )
                .build();

            kb_config_builder = kb_config_builder.retrieval_configuration(retrieval_config);
        }

        let kb_config = kb_config_builder.build().map_err(|e| {
            tracing::error!("Error building KB config: {:?}", e);
            Error::AwsBedrockError(format!("{:?}", e))
        })?;

        let retrieve_config = RetrieveAndGenerateConfiguration::builder()
            .set_type(Some(
                aws_sdk_bedrockagentruntime::types::RetrieveAndGenerateType::KnowledgeBase,
            ))
            .knowledge_base_configuration(kb_config)
            .build()
            .map_err(|e| {
                tracing::error!("Error building retrieve config: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?;

        let input = RetrieveAndGenerateInput::builder()
            .text(query)
            .build()
            .map_err(|e| {
                tracing::error!("Error building input: {:?}", e);
                Error::AwsBedrockError(format!("{:?}", e))
            })?;

        let mut request = self
            .agent_client
            .retrieve_and_generate()
            .retrieve_and_generate_configuration(retrieve_config)
            .input(input);

        if let Some(sid) = session_id {
            request = request.session_id(sid);
        }

        let response = request.send().await.map_err(|e| {
            tracing::error!("Error calling retrieve_and_generate: {:?}", e);
            Error::AwsBedrockError(format!("{:?}", e))
        })?;

        tracing::info!("RetrieveAndGenerate response received");

        let returned_session_id = response.session_id().to_string();
        tracing::info!("Session ID: {}", returned_session_id);

        // Log citations if available
        let citations = response.citations();
        if citations.is_empty() {
            tracing::warn!(
                "No citations in response - Knowledge Base may have no matching documents"
            );
        } else {
            tracing::info!("Number of citations: {}", citations.len());
            for (i, citation) in citations.iter().enumerate() {
                let refs = citation.retrieved_references();
                tracing::info!("Citation {}: {} references", i, refs.len());
                for (j, ref_item) in refs.iter().enumerate() {
                    if let Some(location) = ref_item.location() {
                        tracing::info!("  Reference {}: type={:?}", j, location.r#type());
                        if let Some(s3) = location.s3_location() {
                            tracing::info!("    S3 URI: {:?}", s3.uri());
                        }
                    }
                }
            }
        }

        let output_text = response
            .output()
            .ok_or(Error::AwsBedrockError("No output in response".to_string()))?
            .text()
            .to_string();

        Ok((output_text, returned_session_id))
    }
}
