const {
  BedrockAgentClient,
  IngestKnowledgeBaseDocumentsCommand,
} = require("@aws-sdk/client-bedrock-agent");
const { S3Client, HeadObjectCommand } = require("@aws-sdk/client-s3");

function parseSpacePkFromKey(key) {
  const parts = key.split("/").filter(Boolean);
  if (parts.length < 5) return null;
  if (parts[1] !== "spaces") return null;

  const stage = parts[0];
  const spacePk = parts[2];
  const kind = parts[3];

  if (!stage || !spacePk || !kind) return null;
  return { stage, spacePk, kind };
}

exports.handler = async (event) => {
  const bedrockClient = new BedrockAgentClient({
    region: process.env.AWS_REGION,
  });
  const s3Client = new S3Client({ region: process.env.AWS_REGION });

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
    console.log(`Skipping file outside prefix '${prefix}': ${key}`);
    return { statusCode: 200, body: "Skipped - outside prefix" };
  }

  const parsed = parseSpacePkFromKey(key);
  if (!parsed) {
    console.error(`Invalid key format (cannot parse stage/space_pk): ${key}`);
    return { statusCode: 400, body: "Invalid key format" };
  }
  const { stage, spacePk } = parsed;

  // Check file extension OR ContentType to verify it's a PDF
  const isPdfExtension = key.toLowerCase().endsWith(".pdf");
  let isPdfContentType = false;

  if (!isPdfExtension) {
    try {
      // Files uploaded without extension - check ContentType metadata
      const headResult = await s3Client.send(
        new HeadObjectCommand({ Bucket: bucket, Key: key })
      );
      isPdfContentType = headResult.ContentType === "application/pdf";
      console.log(
        `ContentType: ${headResult.ContentType}, is PDF: ${isPdfContentType}`
      );
    } catch (error) {
      console.error("Error checking file metadata:", error);
      return { statusCode: 500, body: "Error checking file type" };
    }
  }

  const isJsonExtension = key.toLowerCase().endsWith(".json");

  if (!isPdfExtension && !isPdfContentType && !isJsonExtension) {
    console.log(`Skipping unsupported file: ${key}`);
    return { statusCode: 200, body: "Skipped - unsupported file type" };
  }

  console.log("Triggering direct document ingestion (CUSTOM data source)");

  try {
    const s3Uri = `s3://${bucket}/${key}`;

    // Use IngestKnowledgeBaseDocuments for CUSTOM data sources
    const response = await bedrockClient.send(
      new IngestKnowledgeBaseDocumentsCommand({
        knowledgeBaseId: process.env.KNOWLEDGE_BASE_ID,
        dataSourceId: process.env.DATA_SOURCE_ID,
        documents: [
          {
            content: {
              dataSourceType: "CUSTOM",
              custom: {
                customDocumentIdentifier: {
                  id: key,
                },
                sourceType: "S3_LOCATION",
                s3Location: {
                  uri: s3Uri,
                },
              },
            },
            metadata: {
              type: "INLINE",
              inlineAttributes: [
                {
                  key: "space_pk",
                  value: { type: "STRING", stringValue: spacePk },
                },
              ],
            },
          },
        ],
      })
    );

    console.log("Direct document ingestion completed:", {
      failedDocuments: response.failedDocuments?.length || 0,
      documentDetails: response.documentDetails || [],
    });

    if (response.failedDocuments && response.failedDocuments.length > 0) {
      console.error("Failed documents:", response.failedDocuments);
      return {
        statusCode: 500,
        body: JSON.stringify({
          message: "Document ingestion failed",
          errors: response.failedDocuments,
        }),
      };
    }

    return {
      statusCode: 200,
      body: JSON.stringify({
        message: "Direct document ingestion successful",
        documentUri: s3Uri,
        space_pk: spacePk,
        stage,
        details: response.documentDetails,
      }),
    };
  } catch (error) {
    console.error("Error ingesting document:", error);
    throw error;
  }
};
