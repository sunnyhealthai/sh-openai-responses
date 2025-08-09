// tests/client_integration_test.rs

use openai_responses::client::{Client, create_response, create_response_stream};
use openai_responses::models;
use openai_responses::models::responses::{ResponseCreateParams, ResponseInput, ResponseInputItem, ResponseInputItemMessage, ResponseInputText, ResponseStreamEvent, ToolChoice, ToolChoiceOptions, ToolChoiceTypes};
use dotenvy::dotenv;
use std::env;
use std::fs;
use std::error::Error as StdError;
use futures_util::StreamExt;

/// Helper function to initialize the client from environment variables.
/// Panics if the API_KEY is not set.
fn setup_client() -> Client {
    // Load environment variables from .env file
    dotenv().expect("Failed to read .env file. Please create one with your API_KEY.");

    let api_key = env::var("OPENAI_API_KEY")
        .expect("API_KEY not found in environment. Please set it in your .env file.");

    Client::new(api_key)
}

#[tokio::test]
async fn test_create_standard_response() {
    println!("🧪 Running test: Standard Request & Deserialization...");
    let simple_input = vec![
        models::responses::ResponseInputItem::EasyMessage(models::responses::EasyInputMessage {
            role: models::responses::MessageRole::User,
            content: models::responses::EasyInputMessageContent::String("Hello, what is Rust?".to_string()),
            type_field: Some("message".to_string()),
        }),
    ];
    
    // 1. Setup
    let client = setup_client();
    let params = ResponseCreateParams {
        model: Some("gpt-4.1".to_string()), // Use a model available on your endpoint
        stream: None,
        background: None,
        include: None,
        input: Some(models::responses::ResponseInputParam::Items(simple_input)),
        instructions: None,
        max_output_tokens: None,
        metadata: None,
        parallel_tool_calls: None,
        previous_response_id: None,
        prompt: None,
        reasoning: None,
        service_tier: None,
        store: None,
        temperature: None,
        text: None,
        tool_choice: None,
        tools: None,
        top_p: None,
        truncation: None,
        user: None, // The client method will set this to false
    };

    // 2. Act
    let result = create_response(&client, params).await;
    println!("Result: {result:?}");
    // 3. Assert
    assert!(result.is_ok(), "API call failed: {:?}", result.err());

    let response = result.unwrap();
    assert!(!response.id.is_empty(), "Response ID should not be empty.");
    assert!(!response.output.is_empty(), "Response should have at least one choice.");
}

#[tokio::test]
async fn test_create_streaming_response() {
    println!("🧪 Running test: Streaming Request & Deserialization...");
    let simple_input = vec![
        models::responses::ResponseInputItem::EasyMessage(models::responses::EasyInputMessage {
            role: models::responses::MessageRole::User,
            content: models::responses::EasyInputMessageContent::String("Hello, what is Rust?".to_string()),
            type_field: Some("message".to_string()),
        }),
    ];
    // 1. Setup
    let client = setup_client();
    let params = ResponseCreateParams {
        model: Some("gpt-4.1".to_string()), // Use a model available on your endpoint
        stream: Some(true),
        background: None,
        include: None,
        input: Some(models::responses::ResponseInputParam::Items(simple_input)),
        instructions: None,
        max_output_tokens: None,
        metadata: None,
        parallel_tool_calls: None,
        previous_response_id: None,
        prompt: None,
        reasoning: None,
        service_tier: None,
        store: None,
        temperature: None,
        text: None,
        tool_choice: Some(ToolChoice::Option(ToolChoiceOptions::Auto)),
        tools: None,
        top_p: None,
        truncation: None,
        user: None, // The client method will set this to false
    };


    // 2. Act
    let stream_result = create_response_stream(&client, params).await;
    
    // 3. Assert
    assert!(stream_result.is_ok(), "Failed to create stream: {:?}", stream_result.err());
    
    let mut stream = Box::pin(stream_result.unwrap()); // Pin the stream to enable .next()
    
    let mut chunk_count = 0;
    let mut full_content = String::new();

    while let Some(item) = stream.next().await {
        assert!(item.is_ok(), "Received an error from the stream: {:?}", item.err());
        let chunk = item.unwrap();
        chunk_count += 1;

        // Extract content based on the specific stream event type
        match chunk {
            ResponseStreamEvent::ResponseTextDelta(delta_event) => {
                full_content.push_str(&delta_event.delta);
            }
            ResponseStreamEvent::ResponseCompleted(completed_event) => {
                // Optionally handle completion event
                println!("Stream completed with response ID: {}", completed_event.response.id);
            }
            _ => {
                // Handle other event types as needed, or ignore them
            }
        }
    }
    
    assert!(chunk_count > 0, "Stream did not return any chunks.");
    assert!(!full_content.is_empty(), "Stream did not produce any content.");

    println!("✅ Streaming Response OK. Received {} chunks.", chunk_count);
    println!("Final assembled content:\n{}", full_content);
}

#[tokio::test]
async fn test_create_tools_response() {
    println!("🧪 Running test: Tools Request & Response...");
    
    // Define a simple tool using the flat structure
    let tools = vec![
        models::responses::Tool::Function(models::responses::FunctionTool {
            type_field: "function".to_string(),
            name: "get_weather".to_string(),
            description: Some("Get the current weather for a location".to_string()),
            parameters: {
                let schema = serde_json::json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city and state, e.g. San Francisco, CA"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The temperature unit to use"
                        }
                    },
                    "required": ["location"]
                });
                if let serde_json::Value::Object(map) = schema {
                    Some(map.into_iter().collect())
                } else {
                    None
                }
            },
            strict: None,
        })
    ];

    let simple_input = vec![
        models::responses::ResponseInputItem::EasyMessage(models::responses::EasyInputMessage {
            role: models::responses::MessageRole::User,
            content: models::responses::EasyInputMessageContent::String("What's the weather like in San Francisco?".to_string()),
            type_field: Some("message".to_string()),
        }),
    ];
    
    // 1. Setup
    let client = setup_client();
    let params = ResponseCreateParams {
        model: Some("gpt-4.1".to_string()),
        stream: None,
        background: None,
        include: None,
        input: Some(models::responses::ResponseInputParam::Items(simple_input)),
        instructions: None,
        max_output_tokens: None,
        metadata: None,
        parallel_tool_calls: None,
        previous_response_id: None,
        prompt: None,
        reasoning: None,
        service_tier: None,
        store: None,
        temperature: None,
        text: None,
        tool_choice: Some(ToolChoice::Option(ToolChoiceOptions::Auto)),
        tools: Some(tools),
        top_p: None,
        truncation: None,
        user: None,
    };

    // 2. Act
    let result = create_response(&client, params).await;
    
    // 3. Debug and Assert
    match result {
        Ok(response) => {
            println!("✅ API call succeeded");
            assert!(!response.id.is_empty(), "Response ID should not be empty.");
            assert!(!response.output.is_empty(), "Response should have at least one output item.");
            
            // Check if the response contains function tool calls
            let has_function_calls = response.output.iter().any(|output_item| {
                matches!(output_item, models::responses::ResponseOutputItem::FunctionCall(_))
            });
            
            if has_function_calls {
                println!("✅ Response contains function calls as expected.");
                
                // Print function call details for verification
                for output_item in &response.output {
                    if let models::responses::ResponseOutputItem::FunctionCall(function_call) = output_item {
                        println!("Function call: {} with arguments: {}", 
                                function_call.name, 
                                function_call.arguments);
                    }
                }
            } else {
                println!("ℹ️  Response did not trigger function calls, but test completed successfully.");
                // This is still a valid response - the model might choose not to use tools
                
                // Print what output items we did get for debugging
                for (i, output_item) in response.output.iter().enumerate() {
                    match output_item {
                        models::responses::ResponseOutputItem::Message(msg) => {
                            println!("Output {}: Message with role: {}", i, msg.role);
                        }
                        models::responses::ResponseOutputItem::FunctionCall(func) => {
                            println!("Output {}: Function call: {}", i, func.name);
                        }
                        _ => {
                            println!("Output {}: Other type", i);
                        }
                    }
                }
            }
            
            println!("✅ Tools response test completed successfully");
        }
        Err(e) => {
            println!("❌ API call failed with error: {:#?}", e);
            
                         // Try to extract more information about the error
             match &e {
                 openai_responses::client::Error::Reqwest(reqwest_err) => {
                     println!("Reqwest error details:");
                     println!("  Is decode error: {:?}", reqwest_err.is_decode());
                     println!("  Error: {}", reqwest_err);
                     
                     if let Some(source) = reqwest_err.source() {
                         println!("  Source: {}", source);
                     }
                 }
                 _ => {
                     println!("Other error type: {:#?}", e);
                 }
             }
            
            panic!("API call failed: {:?}", e);
        }
    }
}