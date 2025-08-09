// src/models.rs
// File generated from our OpenAPI spec by Stainless. See CONTRIBUTING.md for details.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

//=======================================================================================
// Re-usable & Shared Primitives
//=======================================================================================

/// A helper type for key-value metadata. Keys are strings with a maximum length of 64
/// characters. Values are strings with a maximum length of 512 characters.
pub type Metadata = HashMap<String, String>;

/// Represents a model identifier, e.g., "gpt-4o".
pub type ResponsesModel = String;

/// A list of one or many input items to the model, containing different content types.
pub type ResponseInputMessageContentList = Vec<ResponseInputContent>;

/// A list of one or many input items to the model, containing different content types.
pub type ResponseInput = Vec<ResponseInputItem>;

/// A page of response items, typically used for pagination.
pub type ResponseItemsPage = CursorPage<ResponseItem>;

//=======================================================================================
// Enums from String Literals
//=======================================================================================

/// The type of computer environment to control.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ComputerToolEnvironment {
    Windows,
    Mac,
    Linux,
    Ubuntu,
    Browser,
}

/// The role of a message input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Developer,
}

/// The detail level for an image input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    Low,
    High,
    Auto,
}

/// The status of an item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    InProgress,
    Completed,
    Incomplete,
    Interpreting,
    Failed,
    Searching,
    Generating,
    Queued,
    Cancelled,
}

/// The status of the response generation.
pub type ResponseStatus = ItemStatus;

/// The reason why a response is incomplete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteDetailsReason {
    MaxOutputTokens,
    ContentFilter,
}

/// The latency tier for processing a request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
    Scale,
    Priority,
}

/// The truncation strategy for a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TruncationStrategy {
    Auto,
    Disabled,
}

/// Error codes for a failed response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseErrorCode {
    ServerError,
    RateLimitExceeded,
    InvalidPrompt,
    VectorStoreTimeout,
    InvalidImage,
    InvalidImageFormat,
    InvalidBase64Image,
    InvalidImageUrl,
    ImageTooLarge,
    ImageTooSmall,
    ImageParseError,
    ImageContentPolicyViolation,
    InvalidImageMode,
    ImageFileTooLarge,
    UnsupportedImageMediaType,
    EmptyImageFile,
    FailedToDownloadImage,
    ImageFileNotFound,
}

/// Controls which (if any) tool is called by the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceOptions {
    None,
    Auto,
    Required,
}

/// Specifies additional output data to include in the model response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseIncludable {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
}

//=======================================================================================
// Core Struct and Enum Definitions
//=======================================================================================

/// A text input to the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputText {
    /// The text input to the model.
    pub text: String,
    /// The type of the input item. Always `input_text`.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// An image input to the model. Learn about [image inputs](https://platform.openai.com/docs/guides/vision).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputImage {
    /// The detail level of the image to be sent to the model.
    pub detail: ImageDetail,
    /// The type of the input item. Always `input_image`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// A file input to the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputFile {
    /// The type of the input item. Always `input_file`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The content of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    /// The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// The name of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// Multi-modal input contents.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputContent {
    InputText(ResponseInputText),
    InputImage(ResponseInputImage),
    InputFile(ResponseInputFile),
}

/// Placeholder for pagination structure.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CursorPage<T> {
    pub data: Vec<T>,
    // other pagination fields like 'has_more', 'next_cursor', etc. would go here
}

/// Placeholder for a comparison filter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComparisonFilter {}

/// Placeholder for a compound filter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompoundFilter {}

/// A filter to apply.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Filter {
    Comparison(ComparisonFilter),
    Compound(CompoundFilter),
}

/// A tool that searches for relevant content from uploaded files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileSearchTool {
    /// The type of the file search tool. Always `file_search`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The IDs of the vector stores to search.
    pub vector_store_ids: Vec<String>,
    /// A filter to apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filter>,
    /// The maximum number of results to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_num_results: Option<i64>,
    /// Ranking options for search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_options: Option<FileSearchToolRankingOptions>,
}

/// Ranking options for search.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileSearchToolRankingOptions {
    /// The ranker to use for the file search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranker: Option<String>,
    /// The score threshold for the file search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
}

/// Defines a function in your own code the model can choose to call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FunctionTool {
    /// The name of the function to call.
    pub name: String,
    /// A JSON schema object describing the parameters of the function.
    pub parameters: Option<HashMap<String, Value>>,
    /// Whether to enforce strict parameter validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    /// The type of the function tool. Always `function`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// A description of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// This tool searches the web for relevant results to use in a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebSearchTool {
    /// The type of the web search tool.
    #[serde(rename = "type")]
    pub type_field: String,
    /// High level guidance for the amount of context window space to use for the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<String>,
    /// The user's location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<WebSearchToolUserLocation>,
}

/// The user's location.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WebSearchToolUserLocation {
    /// The type of location approximation. Always `approximate`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Free text input for the city of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The two-letter ISO country code of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Free text input for the region of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The IANA timezone of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// A tool that controls a virtual computer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ComputerTool {
    /// The height of the computer display.
    pub display_height: f64,
    /// The width of the computer display.
    pub display_width: f64,
    /// The type of computer environment to control.
    pub environment: ComputerToolEnvironment,
    /// The type of the computer use tool.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// Give the model access to additional tools via remote Model Context Protocol (MCP) servers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolMcp {
    /// A label for this MCP server, used to identify it in tool calls.
    pub server_label: String,
    /// The URL for the MCP server.
    pub server_url: String,
    /// The type of the MCP tool. Always `mcp`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// List of allowed tool names or a filter object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<ToolMcpAllowedTools>,
    /// Optional HTTP headers to send to the MCP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Specify which of the MCP server's tools require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<ToolMcpRequireApproval>,
    /// Optional description of the MCP server, used to provide more context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolMcpAllowedTools {
    Names(Vec<String>),
    Filter(ToolMcpAllowedToolsFilter),
}

/// A filter object to specify which tools are allowed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolMcpAllowedToolsFilter {
    /// List of allowed tool names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolMcpRequireApproval {
    Always(String), // "always"
    Never(String),  // "never"
    Filter(ToolMcpToolApprovalFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolMcpToolApprovalFilter {
    /// A list of tools that always require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub always: Option<ToolMcpToolApprovalFilterAlways>,
    /// A list of tools that never require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never: Option<ToolMcpToolApprovalFilterNever>,
}

/// A list of tools that always require approval.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolMcpToolApprovalFilterAlways {
    /// List of tools that require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

/// A list of tools that never require approval.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolMcpToolApprovalFilterNever {
    /// List of tools that do not require approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

/// A tool that runs Python code to help generate a response to a prompt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCodeInterpreter {
    /// The code interpreter container.
    pub container: ToolCodeInterpreterContainer,
    /// The type of the code interpreter tool.
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolCodeInterpreterContainer {
    Id(String),
    Auto(ToolCodeInterpreterToolAuto),
}

/// Configuration for a code interpreter container.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCodeInterpreterToolAuto {
    /// Always `auto`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// An optional list of uploaded files to make available to your code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
}

/// A tool that generates images using a model like `gpt-image-1`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolImageGeneration {
    /// The type of the image generation tool.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Background type for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Optional mask for inpainting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_image_mask: Option<ToolImageGenerationInputImageMask>,
    /// The image generation model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Moderation level for the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<String>,
    /// Compression level for the output image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<f64>,
    /// The output format of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// Number of partial images to generate in streaming mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<f64>,
    /// The quality of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    /// The size of the generated image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

/// Optional mask for inpainting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolImageGenerationInputImageMask {
    /// File ID for the mask image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Base64-encoded mask image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// A tool that allows the model to execute shell commands in a local environment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolLocalShell {
    /// The type of the local shell tool.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A tool that can be used to generate a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Tool {
    Function(FunctionTool),
    FileSearch(FileSearchTool),
    #[serde(rename = "web_search_preview")]
    WebSearchPreview(WebSearchTool),
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311(WebSearchTool),
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview(ComputerTool),
    Mcp(ToolMcp),
    CodeInterpreter(ToolCodeInterpreter),
    ImageGeneration(ToolImageGeneration),
    LocalShell(ToolLocalShell),
}

/// An error object returned when the model fails to generate a Response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseError {
    /// The error code for the response.
    pub code: ResponseErrorCode,
    /// A human-readable description of the error.
    pub message: String,
}

/// Details about why the response is incomplete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseIncompleteDetails {
    /// The reason why the response is incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<IncompleteDetailsReason>,
}

/// Use this option to force the model to call a specific function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    pub name: String,
    /// For function calling, the type is always `function`.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// Indicates that the model should use a built-in tool to generate a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolChoiceTypes {
    #[serde(rename = "type")]
    pub type_field: String,
}

/// How the model should select which tool (or tools) to use.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    Option(ToolChoiceOptions),
    Type(ToolChoiceTypes),
    Function(ToolChoiceFunction),
}

/// Represents token usage details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseUsage {
    /// The number of input tokens.
    pub input_tokens: i64,
    /// A detailed breakdown of the input tokens.
    pub input_tokens_details: ResponseUsageInputTokensDetails,
    /// The number of output tokens.
    pub output_tokens: i64,
    /// A detailed breakdown of the output tokens.
    pub output_tokens_details: ResponseUsageOutputTokensDetails,
    /// The total number of tokens used.
    pub total_tokens: i64,
}

/// A detailed breakdown of the input tokens.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseUsageInputTokensDetails {
    /// The number of tokens that were retrieved from the cache.
    pub cached_tokens: i64,
}

/// A detailed breakdown of the output tokens.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseUsageOutputTokensDetails {
    /// The number of reasoning tokens.
    pub reasoning_tokens: i64,
}

/// A citation to a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextFileCitation {
    /// The ID of the file.
    pub file_id: String,
    /// The filename of the file cited.
    pub filename: String,
    /// The index of the file in the list of files.
    pub index: u64,
}

/// A citation for a web resource used to generate a model response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextURLCitation {
    /// The index of the last character of the URL citation in the message.
    pub end_index: u64,
    /// The index of the first character of the URL citation in the message.
    pub start_index: u64,
    /// The title of the web resource.
    pub title: String,
    /// The URL of the web resource.
    pub url: String,
}

/// A citation for a container file used to generate a model response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextContainerFileCitation {
    /// The ID of the container file.
    pub container_id: String,
    /// The index of the last character of the container file citation in the message.
    pub end_index: u64,
    /// The ID of the file.
    pub file_id: String,
    /// The filename of the container file cited.
    pub filename: String,
    /// The index of the first character of the container file citation in the message.
    pub start_index: u64,
}

/// A path to a file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextFilePath {
    /// The ID of the file.
    pub file_id: String,
    /// The index of the file in the list of files.
    pub index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputTextAnnotation {
    FileCitation(ResponseOutputTextFileCitation),
    UrlCitation(ResponseOutputTextURLCitation),
    ContainerFileCitation(ResponseOutputTextContainerFileCitation),
    FilePath(ResponseOutputTextFilePath),
}

/// The log probability of a token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextLogprob {
    pub token: String,
    pub bytes: Vec<u64>,
    pub logprob: f64,
    pub top_logprobs: Vec<ResponseOutputTextLogprobTopLogprob>,
}

/// The top log probability of a token.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextLogprobTopLogprob {
    pub token: String,
    pub bytes: Vec<u64>,
    pub logprob: f64,
}

/// A text output from the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputText {
    /// The annotations of the text output.
    pub annotations: Vec<ResponseOutputTextAnnotation>,
    /// The text output from the model.
    pub text: String,
    /// The type of the output text. Always `output_text`.
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<ResponseOutputTextLogprob>>,
}

/// A refusal from the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputRefusal {
    /// The refusal explanation from the model.
    pub refusal: String,
    /// The type of the refusal. Always `refusal`.
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseContent {
    OutputText(ResponseOutputText),
    OutputRefusal(ResponseOutputRefusal),
    InputText(ResponseInputText),
    InputImage(ResponseInputImage),
    InputFile(ResponseInputFile),
}

/// An output message from the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputMessage {
    /// The unique ID of the output message.
    pub id: String,
    /// The content of the output message.
    pub content: Vec<ResponseOutputContentChoice>,
    /// The role of the output message. Always `assistant`.
    pub role: String,
    /// The status of the message input.
    pub status: ItemStatus,
    /// The type of the output message. Always `message`.
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseOutputContentChoice {
    Text(ResponseOutputText),
    Refusal(ResponseOutputRefusal),
}

/// The results of a file search tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchToolCall {
    /// The unique ID of the file search tool call.
    pub id: String,
    /// The queries used to search for files.
    pub queries: Vec<String>,
    /// The status of the file search tool call.
    pub status: ItemStatus,
    /// The type of the file search tool call. Always `file_search_call`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The results of the file search tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ResponseFileSearchToolCallResult>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchToolCallResult {
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, Value>>,
    /// The unique ID of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The relevance score of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// The text that was retrieved from the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// A tool call to run a function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionToolCall {
    /// A JSON string of the arguments to pass to the function.
    pub arguments: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The name of the function to run.
    pub name: String,
    /// The type of the function tool call. Always `function_call`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The unique ID of the function tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// The results of a web search tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionWebSearch {
    /// The unique ID of the web search tool call.
    pub id: String,
    /// The status of the web search tool call.
    pub status: ItemStatus,
    /// The type of the web search tool call.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A click action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallClick {
    /// Indicates which mouse button was pressed during the click.
    pub button: String,
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The x-coordinate where the click occurred.
    pub x: f64,
    /// The y-coordinate where the click occurred.
    pub y: f64,
}

/// A double click action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallDoubleClick {
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The x-coordinate where the double click occurred.
    pub x: f64,
    /// The y-coordinate where the double click occurred.
    pub y: f64,
}

/// A drag action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallDrag {
    /// An array of coordinates representing the path of the drag action.
    pub path: Vec<ResponseComputerToolCallDragPath>,
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A series of x/y coordinate pairs in the drag path.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallDragPath {
    /// The x-coordinate.
    pub x: f64,
    /// The y-coordinate.
    pub y: f64,
}

/// A collection of keypresses the model would like to perform.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallKeypress {
    /// The combination of keys the model is requesting to be pressed.
    pub keys: Vec<String>,
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A mouse move action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallMove {
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The x-coordinate to move to.
    pub x: f64,
    /// The y-coordinate to move to.
    pub y: f64,
}

/// A screenshot action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallScreenshot {
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A scroll action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallScroll {
    /// The horizontal scroll distance.
    pub scroll_x: f64,
    /// The vertical scroll distance.
    pub scroll_y: f64,
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The x-coordinate where the scroll occurred.
    pub x: f64,
    /// The y-coordinate where the scroll occurred.
    pub y: f64,
}

/// An action to type in text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallType {
    /// The text to type.
    pub text: String,
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A wait action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallWait {
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseComputerToolCallAction {
    Click(ResponseComputerToolCallClick),
    DoubleClick(ResponseComputerToolCallDoubleClick),
    Drag(ResponseComputerToolCallDrag),
    Keypress(ResponseComputerToolCallKeypress),
    Move(ResponseComputerToolCallMove),
    Screenshot(ResponseComputerToolCallScreenshot),
    Scroll(ResponseComputerToolCallScroll),
    Type(ResponseComputerToolCallType),
    Wait(ResponseComputerToolCallWait),
}

/// A pending safety check for the computer call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallPendingSafetyCheck {
    /// The ID of the pending safety check.
    pub id: String,
    /// The type of the pending safety check.
    pub code: String,
    /// Details about the pending safety check.
    pub message: String,
}

/// A tool call to a computer use tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCall {
    /// The unique ID of the computer call.
    pub id: String,
    /// A click action.
    pub action: ResponseComputerToolCallAction,
    /// An identifier used when responding to the tool call with output.
    pub call_id: String,
    /// The pending safety checks for the computer call.
    pub pending_safety_checks: Vec<ResponseComputerToolCallPendingSafetyCheck>,
    /// The status of the item.
    pub status: ItemStatus,
    /// The type of the computer call.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A description of the chain of thought used by a reasoning model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningItem {
    /// The unique identifier of the reasoning content.
    pub id: String,
    /// Reasoning text contents.
    pub summary: Vec<ResponseReasoningItemSummary>,
    /// The type of the object. Always `reasoning`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The encrypted content of the reasoning item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningItemSummary {
    /// A short summary of the reasoning used by the model when generating the response.
    pub text: String,
    /// The type of the object. Always `summary_text`.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// An image generation request made by the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageGenerationCall {
    /// The unique ID of the image generation call.
    pub id: String,
    /// The generated image encoded in base64.
    pub result: Option<String>,
    /// The status of the image generation call.
    pub status: ItemStatus,
    /// The type of the image generation call.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// The logs output from the code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterToolCallLogs {
    /// The logs output from the code interpreter.
    pub logs: String,
    /// The type of the output. Always 'logs'.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// The image output from the code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterToolCallImage {
    /// The type of the output. Always 'image'.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The URL of the image output from the code interpreter.
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseCodeInterpreterToolCallOutput {
    Logs(ResponseCodeInterpreterToolCallLogs),
    Image(ResponseCodeInterpreterToolCallImage),
}

/// A tool call to run code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterToolCall {
    /// The unique ID of the code interpreter tool call.
    pub id: String,
    /// The code to run, or null if not available.
    pub code: Option<String>,
    /// The ID of the container used to run the code.
    pub container_id: String,
    /// The outputs generated by the code interpreter.
    pub outputs: Option<Vec<ResponseCodeInterpreterToolCallOutput>>,
    /// The status of the code interpreter tool call.
    pub status: ItemStatus,
    /// The type of the code interpreter tool call.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// A tool call to run a command on the local shell.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalShellCall {
    /// The unique ID of the local shell call.
    pub id: String,
    /// Execute a shell command on the server.
    pub action: LocalShellCallAction,
    /// The unique ID of the local shell tool call generated by the model.
    pub call_id: String,
    /// The status of the local shell call.
    pub status: ItemStatus,
    /// The type of the local shell call.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// Execute a shell command on the server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalShellCallAction {
    /// The command to run.
    pub command: Vec<String>,
    /// Environment variables to set for the command.
    pub env: HashMap<String, String>,
    /// The type of the local shell action. Always `exec`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Optional timeout in milliseconds for the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<f64>,
    /// Optional user to run the command as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Optional working directory to run the command in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// An invocation of a tool on an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpCall {
    /// The unique ID of the tool call.
    pub id: String,
    /// A JSON string of the arguments passed to the tool.
    pub arguments: String,
    /// The name of the tool that was run.
    pub name: String,
    /// The label of the MCP server running the tool.
    pub server_label: String,
    /// The type of the item. Always `mcp_call`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The error from the tool call, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The output from the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// A list of tools available on an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpListTools {
    /// The unique ID of the list.
    pub id: String,
    /// The label of the MCP server.
    pub server_label: String,
    /// The tools available on the server.
    pub tools: Vec<McpListToolsTool>,
    /// The type of the item. Always `mcp_list_tools`.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Error message if the server could not list tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// A tool available on an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpListToolsTool {
    /// The JSON schema describing the tool's input.
    pub input_schema: Value,
    /// The name of the tool.
    pub name: String,
    /// Additional annotations about the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Value>,
    /// The description of the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A request for human approval of a tool invocation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpApprovalRequest {
    /// The unique ID of the approval request.
    pub id: String,
    /// A JSON string of arguments for the tool.
    pub arguments: String,
    /// The name of the tool to run.
    pub name: String,
    /// The label of the MCP server making the request.
    pub server_label: String,
    /// The type of the item. Always `mcp_approval_request`.
    #[serde(rename = "type")]
    pub type_field: String,
}

/// An output item from the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseOutputItem {
    Message(ResponseOutputMessage),
    FileSearchCall(ResponseFileSearchToolCall),
    FunctionCall(ResponseFunctionToolCall),
    WebSearchCall(ResponseFunctionWebSearch),
    ComputerCall(ResponseComputerToolCall),
    Reasoning(ResponseReasoningItem),
    ImageGenerationCall(ImageGenerationCall),
    CodeInterpreterCall(ResponseCodeInterpreterToolCall),
    LocalShellCall(LocalShellCall),
    McpCall(McpCall),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
}

/// Represents the main response from the API.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Response {
    /// Unique identifier for this Response.
    pub id: String,
    /// Unix timestamp (in seconds) of when this Response was created.
    pub created_at: i64,
    /// The text output from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
    /// An error object returned when the model fails to generate a Response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    /// Details about why the response is incomplete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<ResponseIncompleteDetails>,
    /// A system (or developer) message inserted into the model's context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<ResponseInstructions>,
    /// Set of 16 key-value pairs that can be attached to an object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Model ID used to generate the response.
    pub model: ResponsesModel,
    /// The object type of this resource - always set to `response`.
    pub object: String,
    /// An array of content items generated by the model.
    pub output: Vec<ResponseOutputItem>,
    /// Whether to allow the model to run tool calls in parallel.
    pub parallel_tool_calls: bool,
    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// How the model should select which tool (or tools) to use.
    pub tool_choice: ToolChoice,
    /// An array of tools the model may call while generating a response.
    pub tools: Vec<Tool>,
    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Whether to run the model response in the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    /// An upper bound for the number of tokens that can be generated for a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    /// The unique ID of the previous response to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Reference to a prompt template and its variables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,
    /// Configuration options for reasoning models.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,
    /// Specifies the latency tier to use for processing the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    /// The status of the response generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ResponseStatus>,
    /// Configuration options for a text response from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    /// The truncation strategy to use for the model response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationStrategy>,
    /// Represents token usage details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponseUsage>,
    /// A stable identifier for your end-users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseInstructions {
    String(String),
    Items(ResponseInput),
}

/// Reference to a prompt template and its variables.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsePrompt {
    /// The unique identifier of the prompt template to use.
    pub id: String,
    /// Optional map of values to substitute in for variables in your prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, ResponsePromptVariable>>,
    /// Optional version of the prompt template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsePromptVariable {
    String(String),
    InputText(ResponseInputText),
    InputImage(ResponseInputImage),
    InputFile(ResponseInputFile),
}

/// The default text response format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormatText {
    #[serde(rename = "type")]
    pub type_field: String,
}

/// The legacy JSON object response format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormatJSONObject {
    #[serde(rename = "type")]
    pub type_field: String,
}

/// JSON Schema response format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormatTextJSONSchemaConfig {
    /// The name of the response format.
    pub name: String,
    /// The schema for the response format.
    pub schema: HashMap<String, Value>,
    /// The type of response format being defined.
    #[serde(rename = "type")]
    pub type_field: String,
    /// A description of what the response format is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether to enable strict schema adherence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// An object specifying the format that the model must output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseFormatTextConfig {
    Text(ResponseFormatText),
    JsonSchema(ResponseFormatTextJSONSchemaConfig),
    JsonObject(ResponseFormatJSONObject),
}

/// Configuration options for a text response from the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseTextConfig {
    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ResponseFormatTextConfig>,
}

/// A message input to the model with a role, which can be a simplified string content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EasyInputMessage {
    /// Text, image, or audio input to the model.
    pub content: EasyInputMessageContent,
    /// The role of the message input.
    pub role: MessageRole,
    /// The type of the message input. Always `message`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EasyInputMessageContent {
    String(String),
    List(ResponseInputMessageContentList),
}

/// A message input to the model with a role and detailed content list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemMessage {
    /// A list of one or many input items to the model.
    pub content: ResponseInputMessageContentList,
    /// The role of the message input.
    pub role: MessageRole,
    /// The status of item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// The type of the message input.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

/// The output of a computer tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemComputerCallOutput {
    /// The ID of the computer tool call that produced the output.
    pub call_id: String,
    /// A computer screenshot image used with the computer use tool.
    pub output: ResponseComputerToolCallOutputScreenshot,
    /// The type of the computer tool call output.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The ID of the computer tool call output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The safety checks reported by the API that have been acknowledged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_safety_checks: Option<Vec<ResponseInputItemComputerCallOutputAcknowledgedSafetyCheck>>,
    /// The status of the message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A pending safety check for the computer call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemComputerCallOutputAcknowledgedSafetyCheck {
    /// The ID of the pending safety check.
    pub id: String,
    /// The type of the pending safety check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Details about the pending safety check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// The output of a function tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemFunctionCallOutput {
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// A JSON string of the output of the function tool call.
    pub output: String,
    /// The type of the function tool call output.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The unique ID of the function tool call output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// The output of a local shell tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemLocalShellCallOutput {
    /// The unique ID of the local shell tool call generated by the model.
    pub id: String,
    /// A JSON string of the output of the local shell tool call.
    pub output: String,
    /// The type of the local shell tool call output.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A response to an MCP approval request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemMcpApprovalResponse {
    /// The ID of the approval request being answered.
    pub approval_request_id: String,
    /// Whether the request was approved.
    pub approve: bool,
    /// The type of the item.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The unique ID of the approval response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional reason for the decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// An internal identifier for an item to reference.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputItemItemReference {
    /// The ID of the item to reference.
    pub id: String,
    /// The type of item to reference.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

/// An item in a response input sequence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseInputItem {
    EasyMessage(EasyInputMessage),
    Message(ResponseInputItemMessage),
    OutputMessage(ResponseOutputMessage),
    FileSearchCall(ResponseFileSearchToolCall),
    ComputerCall(ResponseComputerToolCall),
    ComputerCallOutput(ResponseInputItemComputerCallOutput),
    WebSearchCall(ResponseFunctionWebSearch),
    FunctionCall(ResponseFunctionToolCall),
    FunctionCallOutput(ResponseInputItemFunctionCallOutput),
    Reasoning(ResponseReasoningItem),
    ImageGenerationCall(ImageGenerationCall),
    CodeInterpreterCall(ResponseCodeInterpreterToolCall),
    LocalShellCall(LocalShellCall),
    LocalShellCallOutput(ResponseInputItemLocalShellCallOutput),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
    McpApprovalResponse(ResponseInputItemMcpApprovalResponse),
    McpCall(McpCall),
    ItemReference(ResponseInputItemItemReference),
}

/// A computer screenshot image used with the computer use tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallOutputScreenshot {
    /// Specifies the event type.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The identifier of an uploaded file that contains the screenshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the screenshot image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// The output of a computer tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallOutputItem {
    /// The unique ID of the computer call tool output.
    pub id: String,
    /// The ID of the computer tool call that produced the output.
    pub call_id: String,
    /// A computer screenshot image used with the computer use tool.
    pub output: ResponseComputerToolCallOutputScreenshot,
    /// The type of the computer tool call output.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The safety checks reported by the API that have been acknowledged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged_safety_checks: Option<Vec<ResponseComputerToolCallOutputItemAcknowledgedSafetyCheck>>,
    /// The status of the message input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A pending safety check for the computer call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseComputerToolCallOutputItemAcknowledgedSafetyCheck {
    /// The ID of the pending safety check.
    pub id: String,
    /// The type of the pending safety check.
    pub code: String,
    /// Details about the pending safety check.
    pub message: String,
}

/// A tool call to run a function, with a required ID.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionToolCallItem {
    /// The unique ID of the function tool call.
    pub id: String,
    /// A JSON string of the arguments to pass to the function.
    pub arguments: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The name of the function to run.
    pub name: String,
    /// The type of the function tool call.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// The output of a function tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionToolCallOutputItem {
    /// The unique ID of the function call tool output.
    pub id: String,
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// A JSON string of the output of the function tool call.
    pub output: String,
    /// The type of the function tool call output.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A message input with a required ID.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInputMessageItem {
    /// The unique ID of the message input.
    pub id: String,
    /// A list of one or many input items to the model.
    pub content: ResponseInputMessageContentList,
    /// The role of the message input.
    pub role: MessageRole,
    /// The status of item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// The type of the message input.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
}

/// The output of a local shell tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalShellCallOutputItem {
    /// The unique ID of the local shell tool call generated by the model.
    pub id: String,
    /// A JSON string of the output of the local shell tool call.
    pub output: String,
    /// The type of the local shell tool call output.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The status of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A response to an MCP approval request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpApprovalResponseItem {
    /// The unique ID of the approval response
    pub id: String,
    /// The ID of the approval request being answered.
    pub approval_request_id: String,
    /// Whether the request was approved.
    pub approve: bool,
    /// The type of the item.
    #[serde(rename = "type")]
    pub type_field: String,
    /// Optional reason for the decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Content item used to generate a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseItem {
    Message(ResponseInputMessageItem),
    OutputMessage(ResponseOutputMessage),
    FileSearchCall(ResponseFileSearchToolCall),
    ComputerCall(ResponseComputerToolCall),
    ComputerCallOutput(ResponseComputerToolCallOutputItem),
    WebSearchCall(ResponseFunctionWebSearch),
    FunctionCall(ResponseFunctionToolCallItem),
    FunctionCallOutput(ResponseFunctionToolCallOutputItem),
    ImageGenerationCall(ImageGenerationCall),
    CodeInterpreterCall(ResponseCodeInterpreterToolCall),
    LocalShellCall(LocalShellCall),
    LocalShellCallOutput(LocalShellCallOutputItem),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
    McpApprovalResponse(McpApprovalResponseItem),
    McpCall(McpCall),
}

//=======================================================================================
// Stream Event Structs & Enum
//=======================================================================================

/// Emitted when there is a partial audio response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioDeltaEvent {
    pub delta: String,
    pub sequence_number: u64,
}
/// Emitted when the audio response is complete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioDoneEvent {
    pub sequence_number: u64,
}
/// Emitted when there is a partial transcript of audio.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioTranscriptDeltaEvent {
    pub delta: String,
    pub sequence_number: u64,
}
/// Emitted when the full audio transcript is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseAudioTranscriptDoneEvent {
    pub sequence_number: u64,
}
/// Emitted when a partial code snippet is streamed by the code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    pub delta: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the code snippet is finalized by the code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    pub code: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the code interpreter call is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a code interpreter call is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the code interpreter is actively interpreting the code snippet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the model response is complete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCompletedEvent {
    pub response: Response,
    pub sequence_number: u64,
}
/// Emitted when a new content part is added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseContentPartAddedEvent {
    pub content_index: u64,
    pub item_id: String,
    pub output_index: u64,
    pub part: ResponseOutputContentChoice,
    pub sequence_number: u64,
}
/// Emitted when a content part is done.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseContentPartDoneEvent {
    pub content_index: u64,
    pub item_id: String,
    pub output_index: u64,
    pub part: ResponseOutputContentChoice,
    pub sequence_number: u64,
}
/// An event that is emitted when a response is created.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseCreatedEvent {
    pub nonce: Option<String>,
    pub response: Response,
    pub sequence_number: u64,
}
/// Emitted when an error occurs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseErrorEvent {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    pub sequence_number: u64,
}
/// An event that is emitted when a response fails.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFailedEvent {
    pub response: Response,
    pub sequence_number: u64,
}
/// Emitted when a file search call is completed (results found).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchCallCompletedEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a file search call is initiated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchCallInProgressEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a file search is currently searching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFileSearchCallSearchingEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when there is a partial function-call arguments delta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    pub delta: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when function-call arguments are finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    pub arguments: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when an image generation tool call has completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallCompletedEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when an image generation tool call is actively generating an image.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallGeneratingEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when an image generation tool call is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallInProgressEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a partial image is available during image generation streaming.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseImageGenCallPartialImageEvent {
    pub item_id: String,
    pub output_index: u64,
    pub partial_image_b64: String,
    pub partial_image_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the response is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseInProgressEvent {
    pub response: Response,
    pub sequence_number: u64,
}
/// An event that is emitted when a response finishes as incomplete.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseIncompleteEvent {
    pub response: Response,
    pub sequence_number: u64,
}
/// Emitted when there is a delta to the arguments of an MCP tool call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallArgumentsDeltaEvent {
    pub delta: Value,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the arguments for an MCP tool call are finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallArgumentsDoneEvent {
    pub arguments: Value,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when an MCP tool call has completed successfully.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallCompletedEvent {
    pub sequence_number: u64,
}
/// Emitted when an MCP tool call has failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallFailedEvent {
    pub sequence_number: u64,
}
/// Emitted when an MCP tool call is in progress.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpCallInProgressEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the list of available MCP tools has been successfully retrieved.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpListToolsCompletedEvent {
    pub sequence_number: u64,
}
/// Emitted when the attempt to list available MCP tools has failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpListToolsFailedEvent {
    pub sequence_number: u64,
}
/// Emitted when the system is in the process of retrieving the list of available MCP tools.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseMcpListToolsInProgressEvent {
    pub sequence_number: u64,
}
/// Emitted when a new output item is added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputItemAddedEvent {
    pub item: ResponseOutputItem,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when an output item is marked done.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputItemDoneEvent {
    pub item: ResponseOutputItem,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when an annotation is added to output text content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseOutputTextAnnotationAddedEvent {
    pub annotation: Value,
    pub annotation_index: u64,
    pub content_index: u64,
    #[serde(rename = "item__id")]
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a response is queued and waiting to be processed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseQueuedEvent {
    pub response: Response,
    pub sequence_number: u64,
}
/// Emitted when there is a delta to the reasoning content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningDeltaEvent {
    pub content_index: u64,
    pub delta: Value,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when the reasoning content is finalized for an item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningDoneEvent {
    pub content_index: u64,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
    pub text: String,
}
/// Emitted when there is a delta to the reasoning summary content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryDeltaEvent {
    pub delta: Value,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
    pub summary_index: u64,
}
/// Emitted when the reasoning summary content is finalized for an item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryDoneEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
    pub summary_index: u64,
    pub text: String,
}
/// Emitted when a new reasoning summary part is added.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryPartAddedEvent {
    pub item_id: String,
    pub output_index: u64,
    pub part: ResponseReasoningSummaryPartAddedEventPart,
    pub sequence_number: u64,
    pub summary_index: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryPartAddedEventPart {
    pub text: String,
}
/// Emitted when a reasoning summary part is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryPartDoneEvent {
    pub item_id: String,
    pub output_index: u64,
    pub part: ResponseReasoningSummaryPartDoneEventPart,
    pub sequence_number: u64,
    pub summary_index: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryPartDoneEventPart {
    pub text: String,
}
/// Emitted when a delta is added to a reasoning summary text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    pub delta: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
    pub summary_index: u64,
}
/// Emitted when a reasoning summary text is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseReasoningSummaryTextDoneEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
    pub summary_index: u64,
    pub text: String,
}
/// Emitted when there is a partial refusal text.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseRefusalDeltaEvent {
    pub content_index: u64,
    pub delta: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when refusal text is finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseRefusalDoneEvent {
    pub content_index: u64,
    pub item_id: String,
    pub output_index: u64,
    pub refusal: String,
    pub sequence_number: u64,
}
/// Emitted when there is an additional text delta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseTextDeltaEvent {
    pub content_index: u64,
    pub delta: String,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when text content is finalized.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseTextDoneEvent {
    pub content_index: u64,
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
    pub text: String,
}
/// Emitted when a web search call is completed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWebSearchCallCompletedEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a web search call is initiated.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWebSearchCallInProgressEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}
/// Emitted when a web search call is executing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseWebSearchCallSearchingEvent {
    pub item_id: String,
    pub output_index: u64,
    pub sequence_number: u64,
}

/// Represents all possible events from a response stream.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum ResponseStreamEvent {
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta(ResponseAudioDeltaEvent),
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone(ResponseAudioDoneEvent),
    #[serde(rename = "response.audio_transcript.delta")]
    ResponseAudioTranscriptDelta(ResponseAudioTranscriptDeltaEvent),
    #[serde(rename = "response.audio_transcript.done")]
    ResponseAudioTranscriptDone(ResponseAudioTranscriptDoneEvent),
    #[serde(rename = "response.code_interpreter.code.delta")]
    ResponseCodeInterpreterCallCodeDelta(ResponseCodeInterpreterCallCodeDeltaEvent),
    #[serde(rename = "response.code_interpreter.code.done")]
    ResponseCodeInterpreterCallCodeDone(ResponseCodeInterpreterCallCodeDoneEvent),
    #[serde(rename = "response.code_interpreter.completed")]
    ResponseCodeInterpreterCallCompleted(ResponseCodeInterpreterCallCompletedEvent),
    #[serde(rename = "response.code_interpreter.in_progress")]
    ResponseCodeInterpreterCallInProgress(ResponseCodeInterpreterCallInProgressEvent),
    #[serde(rename = "response.code_interpreter.interpreting")]
    ResponseCodeInterpreterCallInterpreting(ResponseCodeInterpreterCallInterpretingEvent),
    #[serde(rename = "response.completed")]
    ResponseCompleted(ResponseCompletedEvent),
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(ResponseContentPartAddedEvent),
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(ResponseContentPartDoneEvent),
    #[serde(rename = "response.created")]
    ResponseCreated(ResponseCreatedEvent),
    #[serde(rename = "error")]
    Error(ResponseErrorEvent),
    #[serde(rename = "response.file_search.completed")]
    ResponseFileSearchCallCompleted(ResponseFileSearchCallCompletedEvent),
    #[serde(rename = "response.file_search.in_progress")]
    ResponseFileSearchCallInProgress(ResponseFileSearchCallInProgressEvent),
    #[serde(rename = "response.file_search.searching")]
    ResponseFileSearchCallSearching(ResponseFileSearchCallSearchingEvent),
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDeltaEvent),
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsDoneEvent),
    #[serde(rename = "response.in_progress")]
    ResponseInProgress(ResponseInProgressEvent),
    #[serde(rename = "response.failed")]
    ResponseFailed(ResponseFailedEvent),
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete(ResponseIncompleteEvent),
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(ResponseOutputItemAddedEvent),
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(ResponseOutputItemDoneEvent),
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded(ResponseReasoningSummaryPartAddedEvent),
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone(ResponseReasoningSummaryPartDoneEvent),
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta(ResponseReasoningSummaryTextDeltaEvent),
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone(ResponseReasoningSummaryTextDoneEvent),
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta(ResponseRefusalDeltaEvent),
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone(ResponseRefusalDoneEvent),
    #[serde(rename = "response.reasoning_delta")]
    ResponseReasoningDelta(ResponseReasoningDeltaEvent),
    #[serde(rename = "response.reasoning_done")]
    ResponseReasoningDone(ResponseReasoningDoneEvent),
    #[serde(rename = "response.reasoning_summary_delta")]
    ResponseReasoningSummaryDelta(ResponseReasoningSummaryDeltaEvent),
    #[serde(rename = "response.reasoning_summary_done")]
    ResponseReasoningSummaryDone(ResponseReasoningSummaryDoneEvent),
    #[serde(rename = "response.output_text.delta")]
    ResponseTextDelta(ResponseTextDeltaEvent),
    #[serde(rename = "response.output_text.done")]
    ResponseTextDone(ResponseTextDoneEvent),
    #[serde(rename = "response.web_search.completed")]
    ResponseWebSearchCallCompleted(ResponseWebSearchCallCompletedEvent),
    #[serde(rename = "response.web_search.in_progress")]
    ResponseWebSearchCallInProgress(ResponseWebSearchCallInProgressEvent),
    #[serde(rename = "response.web_search.searching")]
    ResponseWebSearchCallSearching(ResponseWebSearchCallSearchingEvent),
    #[serde(rename = "response.image_generation.completed")]
    ResponseImageGenerationCallCompleted(ResponseImageGenCallCompletedEvent),
    #[serde(rename = "response.image_generation.generating")]
    ResponseImageGenerationCallGenerating(ResponseImageGenCallGeneratingEvent),
    #[serde(rename = "response.image_generation.in_progress")]
    ResponseImageGenerationCallInProgress(ResponseImageGenCallInProgressEvent),
    #[serde(rename = "response.image_generation.partial_image")]
    ResponseImageGenerationCallPartialImage(ResponseImageGenCallPartialImageEvent),
    #[serde(rename = "response.mcp.arguments.delta")]
    ResponseMcpCallArgumentsDelta(ResponseMcpCallArgumentsDeltaEvent),
    #[serde(rename = "response.mcp.arguments.done")]
    ResponseMcpCallArgumentsDone(ResponseMcpCallArgumentsDoneEvent),
    #[serde(rename = "response.mcp.completed")]
    ResponseMcpCallCompleted(ResponseMcpCallCompletedEvent),
    #[serde(rename = "response.mcp.failed")]
    ResponseMcpCallFailed(ResponseMcpCallFailedEvent),
    #[serde(rename = "response.mcp.in_progress")]
    ResponseMcpCallInProgress(ResponseMcpCallInProgressEvent),
    #[serde(rename = "response.mcp.list_tools.completed")]
    ResponseMcpListToolsCompleted(ResponseMcpListToolsCompletedEvent),
    #[serde(rename = "response.mcp.list_tools.failed")]
    ResponseMcpListToolsFailed(ResponseMcpListToolsFailedEvent),
    #[serde(rename = "response.mcp.list_tools.in_progress")]
    ResponseMcpListToolsInProgress(ResponseMcpListToolsInProgressEvent),
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded(ResponseOutputTextAnnotationAddedEvent),
    #[serde(rename = "response.queued")]
    ResponseQueued(ResponseQueuedEvent),
}

//=======================================================================================
// Generic Parsed Types
//=======================================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedResponseOutputText<T> {
    #[serde(flatten)]
    pub response_output_text: ResponseOutputText,
    pub parsed: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParsedContent<T> {
    ParsedText(ParsedResponseOutputText<T>),
    Refusal(ResponseOutputRefusal),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedResponseOutputMessage<T> {
    pub id: String,
    pub role: String,
    pub status: ItemStatus,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: Vec<ParsedContent<T>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedResponseFunctionToolCall {
    #[serde(flatten)]
    pub function_tool_call: ResponseFunctionToolCall,
    pub parsed_arguments: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParsedResponseOutputItem<T> {
    Message(ParsedResponseOutputMessage<T>),
    FunctionToolCall(ParsedResponseFunctionToolCall),
    FileSearchCall(ResponseFileSearchToolCall),
    WebSearchCall(ResponseFunctionWebSearch),
    ComputerCall(ResponseComputerToolCall),
    Reasoning(ResponseReasoningItem),
    ImageGenerationCall(ImageGenerationCall),
    CodeInterpreterCall(ResponseCodeInterpreterToolCall),
    LocalShellCall(LocalShellCall),
    McpCall(McpCall),
    McpListTools(McpListTools),
    McpApprovalRequest(McpApprovalRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedResponse<T> {
    #[serde(flatten)]
    pub response: Response,
    pub output: Vec<ParsedResponseOutputItem<T>>,
    pub output_parsed: Option<T>,
}

//=======================================================================================
// Request Parameter Structs
//=======================================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseCreateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<ResponseIncludable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ResponseInputParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ResponsesModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponseInputParam {
    String(String),
    Items(ResponseInput),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseRetrieveParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<ResponseIncludable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

//=======================================================================================
// Chat Completions API Models
//=======================================================================================

/// A message in a chat completion request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    /// The role of the message author.
    pub role: String,
    /// The content of the message.
    pub content: String,
}

/// Parameters for creating a chat completion.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatCompletionCreateParams {
    /// ID of the model to use.
    pub model: String,
    /// A list of messages comprising the conversation so far.
    pub messages: Vec<ChatMessage>,
    /// What sampling temperature to use, between 0 and 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// An alternative to sampling with temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// The maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    /// Whether to stream back partial progress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ChatResponseFormat>,
}

/// The format of the response from a chat completion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatResponseFormat {
    JsonObject(ChatResponseFormatJsonObject),
    JsonSchema(ChatResponseFormatJsonSchema),
}

/// JSON object response format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResponseFormatJsonObject {
    /// The type of response format. Always "json_object".
    #[serde(rename = "type")]
    pub type_field: String,
}

/// JSON schema response format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatResponseFormatJsonSchema {
    /// The type of response format. Always "json_schema".
    #[serde(rename = "type")]
    pub type_field: String,
    /// The JSON schema for the response.
    pub json_schema: ChatJsonSchema,
}

/// JSON schema definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatJsonSchema {
    /// The name of the schema.
    pub name: String,
    /// The schema definition.
    pub schema: HashMap<String, Value>,
    /// Whether to enable strict mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// A choice in a chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatChoice {
    /// The index of the choice.
    pub index: i32,
    /// The message generated by the model.
    pub message: ChatMessage,
    /// The reason the model stopped generating tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Usage statistics for a chat completion.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatUsage {
    /// Number of tokens in the prompt.
    pub prompt_tokens: i32,
    /// Number of tokens in the generated completion.
    pub completion_tokens: i32,
    /// Total number of tokens used.
    pub total_tokens: i32,
}

/// A chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletion {
    /// A unique identifier for the chat completion.
    pub id: String,
    /// The object type, which is always "chat.completion".
    pub object: String,
    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: i64,
    /// The model used for the chat completion.
    pub model: String,
    /// A list of chat completion choices.
    pub choices: Vec<ChatChoice>,
    /// Usage statistics for the completion request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ChatUsage>,
}
