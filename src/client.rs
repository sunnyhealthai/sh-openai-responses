// src/client.rs

//! An asynchronous, production-ready client for the API, built on top of `reqwest`.

use crate::models;
use reqwest::{header, Client as ReqwestClient, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;
use futures_util::{Stream, StreamExt, TryStreamExt};

const API_BASE_URL: &str = "https://api.openai.com/v1";

//=======================================================================================
// Error and Result Types
//=======================================================================================

/// Represents errors that can occur when using the API client.
#[derive(Debug)]
pub enum Error {
    /// An error from the underlying `reqwest` library.
    Reqwest(reqwest::Error),
    /// An error during JSON serialization or deserialization.
    Serde(serde_json::Error),
    /// A specific error returned by the API (e.g., a 4xx or 5xx response).
    ApiError {
        status: StatusCode,
        error: models::responses::ResponseError,
    },
    /// An error for responses that are not successful but don't match the
    /// expected API error format.
    UnexpectedResponse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "Request error: {e}"),
            Error::Serde(e) => write!(f, "Serialization/deserialization error: {e}"),
            Error::ApiError { status, error } => {
                write!(f, "API error (status {}): [{:?}] {}", status, error.code, error.message)
            }
            Error::UnexpectedResponse(msg) => write!(f, "Unexpected API response: {msg}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::Serde(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde(err)
    }
}

/// A convenience type alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;


//=======================================================================================
// Main Client
//=======================================================================================

/// The main client for interacting with the API. It provides access to different
/// API resource groups, such as `responses`.
#[derive(Debug, Clone)]
pub struct Client {
    http_client: ReqwestClient,
    base_url: String,
}

impl Client {
    /// Creates a new API client.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your API key for authentication.
    pub fn new(api_key: String) -> Self {
        Self::new_with_base_url(api_key, API_BASE_URL.to_string())
    }

    /// Creates a new API client with a custom base URL.
    pub fn new_with_base_url(api_key: String, base_url: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {api_key}")).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let http_client = ReqwestClient::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to build ReqwestClient");

        Client {
            http_client,
            base_url,
        }
    }
}

//=======================================================================================
// Response API Functions
//=======================================================================================

/// Creates a model response.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/responses/create)
pub async fn create_response(
    client: &Client,
    mut params: models::responses::ResponseCreateParams,
) -> Result<models::responses::Response> {
    params.stream = Some(false);
    execute_request_with_body(client, Method::POST, "/responses", Some(params)).await
}

/// Creates a model response as a stream of events.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/responses/create)
pub async fn create_response_stream(
    client: &Client,
    mut params: models::responses::ResponseCreateParams,
) -> Result<impl Stream<Item = Result<models::responses::ResponseStreamEvent>>> {
    params.stream = Some(true);
    let request_builder = client.http_client
        .post(format!("{}{}", client.base_url, "/responses"))
        .json(&params);
    execute_stream(request_builder).await
}

/// Retrieves a model response with the given ID.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/responses/retrieve)
pub async fn retrieve_response(
    client: &Client,
    response_id: &str,
    params: Option<models::responses::ResponseRetrieveParams>,
) -> Result<models::responses::Response> {
    let path = format!("/responses/{response_id}");
    let url = format!("{}{}", client.base_url, &path);
    let rb = client.http_client.get(&url).query(&params);
    execute_request(rb).await
}

/// Retrieves a model response as a stream of events.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/responses/retrieve)
pub async fn retrieve_response_stream(
    client: &Client,
    response_id: &str,
    mut params: models::responses::ResponseRetrieveParams,
) -> Result<impl Stream<Item = Result<models::responses::ResponseStreamEvent>>> {
    params.stream = Some(true);
    let path = format!("/responses/{response_id}");
    let url = format!("{}{}", client.base_url, &path);
    let rb = client.http_client.get(&url).query(&params);
    execute_stream(rb).await
}

/// Deletes a model response with the given ID.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/responses/delete)
pub async fn delete_response(client: &Client, response_id: &str) -> Result<()> {
    let path = format!("/responses/{response_id}");
    let response = client.http_client
        .delete(format!("{}{}", client.base_url, &path))
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let error_body = response.text().await?;
        Err(Error::UnexpectedResponse(format!("Status: {status}, Body: {error_body}")))
    }
}

/// Cancels a model response with the given ID.
///
/// Only responses created with the `background` parameter set to `true` can be cancelled.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/responses/cancel)
pub async fn cancel_response(
    client: &Client,
    response_id: &str,
) -> Result<models::responses::Response> {
    let path = format!("/responses/{response_id}/cancel");
    execute_request_with_body(client, Method::POST, &path, None::<()>).await
}

//=======================================================================================
// Chat Completions API Functions
//=======================================================================================

/// Creates a chat completion.
///
/// [API Documentation](https://platform.openai.com/docs/api-reference/chat/create)
pub async fn create_chat_completion(
    client: &Client,
    mut params: models::responses::ChatCompletionCreateParams,
) -> Result<models::responses::ChatCompletion> {
    params.stream = Some(false);
    execute_request_with_body(client, Method::POST, "/chat/completions", Some(params)).await
}

//=======================================================================================
// Helper Functions
//=======================================================================================

/// Helper for standard JSON requests.
async fn execute_request_with_body<T: DeserializeOwned, B: Serialize>(
    client: &Client,
    method: Method,
    path: &str,
    body: Option<B>,
) -> Result<T> {
    let url = format!("{}{}", client.base_url, path);
    let mut request_builder = client.http_client.request(method, &url);

    if let Some(b) = body {
        request_builder = request_builder.json(&b);
    }

    execute_request(request_builder).await
}

/// Executes a pre-built request and handles the response.
async fn execute_request<T: DeserializeOwned>(rb: RequestBuilder) -> Result<T> {
    let response = rb.send().await?;
    let status = response.status();

    if status.is_success() {
        let text_body = response.text().await?;
        let mut deserializer = serde_json::Deserializer::from_str(&text_body);
        match serde_path_to_error::deserialize::<_, T>(&mut deserializer) {
            Ok(json_body) => Ok(json_body),
            Err(err) => {
                eprintln!("[serde_path_to_error] Deserialization error at path: {}", err.path());
                eprintln!("[serde_path_to_error] Error: {err}");
                Err(Error::Serde(err.into_inner()))
            }
        }
    } else {
        let error_body = response.text().await?;
        match serde_json::from_str::<models::responses::ResponseError>(&error_body) {
            Ok(api_error) => Err(Error::ApiError { status, error: api_error }),
            Err(_) => Err(Error::UnexpectedResponse(format!("Status: {status}, Body: {error_body}"))),
        }
    }
}

/// Executes a request and returns a stream of Server-Sent Events.
async fn execute_stream(
    rb: RequestBuilder,
) -> Result<impl Stream<Item = Result<models::responses::ResponseStreamEvent>>> {
    let response = rb.send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await?;
        return match serde_json::from_str::<models::responses::ResponseError>(&error_body) {
            Ok(api_error) => Err(Error::ApiError { status, error: api_error }),
            Err(_) => Err(Error::UnexpectedResponse(format!("Status: {status}, Body: {error_body}"))),
        };
    }

    let byte_stream = response.bytes_stream();

    let event_stream = futures_util::stream::try_unfold(
        (byte_stream, Vec::new()),
        |(mut stream, mut buffer)| async move {
            loop {
                // Check if the buffer contains a complete message (ends with \n\n)
                if let Some(end_of_message_pos) = buffer.windows(2).position(|w| w == b"\n\n") {
                    let message_bytes = buffer.drain(..end_of_message_pos + 2).collect::<Vec<u8>>();
                    let message_str = String::from_utf8_lossy(&message_bytes);
                    
                    let mut data_buf = String::new();
                    for line in message_str.lines() {
                        if let Some(data_line) = line.strip_prefix("data: ") {
                            data_buf.push_str(data_line.trim());
                        }
                    }

                    if data_buf == "[DONE]" {
                        return Ok(None); // Stream is finished
                    }
                    if data_buf.is_empty() {
                        continue; // Skip empty keep-alive messages
                    }

                    // Debug: Print the raw JSON being received
                    println!("🔍 Raw JSON received from stream: {data_buf}");

                    let mut deserializer = serde_json::Deserializer::from_str(&data_buf);
                    let event_result = serde_path_to_error::deserialize::<_, models::responses::ResponseStreamEvent>(&mut deserializer);
                    match event_result {
                        Ok(event) => return Ok(Some((event, (stream, buffer)))),
                        Err(err) => {
                            eprintln!("[serde_path_to_error] Stream deserialization error at path: {}", err.path());
                            eprintln!("[serde_path_to_error] Error: {err}");
                            return Err(Error::Serde(err.into_inner()));
                        }
                    }
                }

                // If not, read more data from the network
                match stream.next().await {
                    Some(Ok(chunk)) => buffer.extend_from_slice(&chunk),
                    Some(Err(e)) => return Err(Error::from(e)),
                    None => { // Stream has ended
                        return if buffer.is_empty() {
                            Ok(None)
                        } else {
                            Err(Error::UnexpectedResponse("Stream ended with incomplete data".into()))
                        };
                    }
                }
            }
        },
    ).map_err(|e| e);

    Ok(event_stream)
}
