# LLM Integration with Chutes AI

This document describes how to integrate and use the LLM (Large Language Model) functionality in the Grocy application using the Chutes AI API.

## Overview

The `llm.rs` module provides integration with the Chutes AI service, specifically using the DeepSeek-V3-0324 model. It supports both streaming and non-streaming chat completions.

## Environment Setup

### Required Environment Variable

You must set the `CHUTES_API_TOKEN` environment variable with your Chutes AI API token:

```bash
export CHUTES_API_TOKEN="your_api_token_here"
```

### Getting a Chutes AI API Token

1. Visit [https://chutes.ai](https://chutes.ai)
2. Sign up for an account
3. Navigate to your API settings
4. Generate an API token
5. Copy the token and set it as your environment variable

## Usage

### Simple Chat

The easiest way to use the LLM is with the `simple_chat` function:

```rust
use crate::llm;

async fn example_usage() {
    match llm::simple_chat("Tell me a joke").await {
        Ok(response) => println!("AI Response: {}", response),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Advanced Chat with Multiple Messages

For more complex conversations, use the `chat_completion` function:

```rust
use crate::llm::{chat_completion, Message};

async fn advanced_chat() {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "What's the weather like?".to_string(),
        },
    ];

    match chat_completion(messages).await {
        Ok(response) => println!("AI Response: {}", response),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Streaming Chat

For real-time streaming responses:

```rust
use crate::llm::{chat_completion_streaming, Message};

async fn streaming_chat() {
    let messages = vec![
        Message {
            role: "user".to_string(),
            content: "Tell me a story".to_string(),
        },
    ];

    let mut full_response = String::new();
    
    match chat_completion_streaming(messages, |chunk| {
        print!("{}", chunk);
        full_response.push_str(&chunk);
    }).await {
        Ok(complete_response) => println!("\nComplete response: {}", complete_response),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Integration in Chat System

The LLM is automatically integrated into the chat system. When users send messages:

1. The user message is stored
2. The `generate_ai_response` function calls the LLM API
3. If the API fails, it falls back to simple keyword-based responses
4. The AI response is stored and displayed

## API Configuration

The module uses the following default configuration:

- **Model**: `deepseek-ai/DeepSeek-V3-0324`
- **Max Tokens**: 1024
- **Temperature**: 0.7
- **Endpoint**: `https://llm.chutes.ai/v1/chat/completions`

## Error Handling

The module provides robust error handling with three main error types:

1. **RequestError**: Network or HTTP-related errors
2. **AuthError**: Authentication issues (missing or invalid API token)
3. **ParseError**: JSON parsing or response format issues

## Testing

To test the LLM integration:

1. Set your `CHUTES_API_TOKEN` environment variable
2. Run the tests:

```bash
cargo test test_simple_chat -- --nocapture
```

3. Or test manually through the chat interface at `http://localhost:3000/ui/chat`

## Fallback Behavior

If the LLM API is unavailable or returns an error, the system falls back to simple keyword-based responses:

- Greetings: "Hello! How can I help you today?"
- Todo-related: Information about todo features
- Help requests: General assistance message
- Default: "That's interesting! Tell me more about it."

## Security Considerations

- **API Token**: Never commit your API token to version control
- **Environment Variables**: Use `.env` files for local development
- **Rate Limiting**: Be aware of API rate limits from Chutes AI
- **Error Logging**: API errors are logged but tokens are not exposed

## Troubleshooting

### Common Issues

1. **"CHUTES_API_TOKEN environment variable not set"**
   - Solution: Set the environment variable as described above

2. **Network timeouts**
   - Solution: Check your internet connection and Chutes AI service status

3. **Authentication errors**
   - Solution: Verify your API token is valid and has sufficient credits

4. **Parse errors**
   - Solution: Check if the API response format has changed

### Debug Logging

Enable debug logging to see detailed LLM interactions:

```bash
RUST_LOG=debug cargo run
```

## Dependencies

The LLM integration requires these dependencies (already added to Cargo.toml):

- `reqwest`: HTTP client for API calls
- `serde`: JSON serialization/deserialization
- `serde_json`: JSON handling
- `futures-util`: Async stream processing
- `tokio`: Async runtime

## Performance Notes

- **Non-streaming responses**: Typically 1-3 seconds depending on response length
- **Streaming responses**: Start immediately, complete faster for user experience
- **Fallback responses**: Instant when LLM API is unavailable
- **Memory usage**: Responses are stored in memory (consider cleanup for production)

## Future Improvements

Potential enhancements for the LLM integration:

1. **Model Selection**: Allow users to choose different models
2. **Response Caching**: Cache common responses to reduce API calls
3. **Context Memory**: Maintain conversation context across sessions
4. **Custom Prompts**: Allow customization of system prompts
5. **Response Filtering**: Content moderation and filtering
6. **Analytics**: Track usage patterns and response quality