use rmcp::{
    ServerHandler,
    model::{CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool,
};

type McpResult = Result<CallToolResult, rmcp::Error>;

#[derive(Clone)]
pub struct RatelMcpServer {}

#[tool(tool_box)]
impl RatelMcpServer {
    pub fn new() -> Self {
        Self {}
    }

    #[tool(description = "Increment the counter by 1")]
    async fn increment(&self) -> McpResult {
        todo!()
    }
}

#[tool(tool_box)]
impl ServerHandler for RatelMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides Ratel functionalities that discuss on a topic and post/read feeds.".to_string()),
        }
    }
}
