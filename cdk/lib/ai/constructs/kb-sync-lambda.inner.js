const {
  BedrockAgentClient,
  StartIngestionJobCommand,
} = require("@aws-sdk/client-bedrock-agent");

exports.handler = async (event) => {
  const client = new BedrockAgentClient({ region: process.env.AWS_REGION });

  console.log("Received EventBridge event:", JSON.stringify(event, null, 2));

  const bucket = event.detail?.bucket?.name;
  const key = event.detail?.object?.key;

  if (!bucket || !key) {
    console.error("Missing bucket or key in event");
    return { statusCode: 400, body: "Invalid event" };
  }

  // Check if file matches the prefix filter (if specified)
  const prefix = process.env.DATA_PREFIX || "";
  if (prefix && !key.startsWith(prefix)) {
    return { statusCode: 200, body: "Skipped - outside prefix" };
  }

  console.log(
    "Triggering ingestion job for Knowledge Base:",
    process.env.KNOWLEDGE_BASE_ID,
  );

  try {
    const response = await client.send(
      new StartIngestionJobCommand({
        knowledgeBaseId: process.env.KNOWLEDGE_BASE_ID,
        dataSourceId: process.env.DATA_SOURCE_ID,
      }),
    );

    console.log("Ingestion job started successfully:", {
      jobId: response.ingestionJob?.ingestionJobId,
      status: response.ingestionJob?.status,
    });

    return {
      statusCode: 200,
      body: JSON.stringify({
        message: "Ingestion triggered successfully",
        jobId: response.ingestionJob?.ingestionJobId,
      }),
    };
  } catch (error) {
    console.error("Error starting ingestion:", error);
    throw error;
  }
};
