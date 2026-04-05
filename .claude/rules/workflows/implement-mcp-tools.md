# Implement MCP Tools

## Step 1: Identify Server Functions to Expose
- Determine which controllers need MCP tool access
- Review existing tools in `app/ratel/src/common/mcp/server.rs` to avoid duplication
- **References**: conventions/mcp-tools.md

## Step 2: Annotate Controllers
- Add `#[mcp_tool(name, description)]` above the route attribute
- Add `#[mcp(description = "...")]` on each parameter
- **References**: conventions/mcp-tools.md, conventions/anti-patterns.md
- **Skills**: rust-mcp-server, rust-knowledge-patch

## Step 3: Register in MCP Server
- Add `#[rmcp::tool]` method in `server.rs` `#[tool_router] impl` block
- Import generated `_mcp_handler` or `_mcp_impl` and `McpRequest` type
- **References**: conventions/mcp-tools.md

## Step 4: Lint & Format
- **References**: conventions/lint-and-format.md

## Step 5: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 6: Test
- Add integration tests in `app/ratel/src/tests/mcp_tests.rs` for each new tool
- Use `setup_mcp_test()`, `mcp_tool_call()`, and `extract_tool_content()` helpers
- **References**: conventions/mcp-tools.md, conventions/build-commands.md
