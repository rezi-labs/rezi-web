# Migration Guide: Single Provider to Multi-Provider Support

## Overview

The Rumors library has been updated to support multiple AI providers (Google Gemini API and Ollama) instead of being locked to Google Gemini API only.

## Breaking Changes

### Constructor Changes

**Before:**
```rust
use rumors::RumorsClient;

let client = RumorsClient::new("your-google-api-key".to_string());
```

**After (Option 1 - New API):**
```rust
use rumors::{RumorsClient, ProviderType};

let client = RumorsClient::new(ProviderType::GoogleGemini {
    api_key: "your-google-api-key".to_string(),
});
```

**After (Option 2 - Backwards Compatibility):**
```rust
use rumors::RumorsClient;

let client = RumorsClient::new_with_google_api("your-google-api-key".to_string());
```

### Error Type Changes

**Before:**
```rust
use rumors::GoogleApiError;

async fn example() -> Result<String, GoogleApiError> {
    // ...
}
```

**After:**
```rust
use rumors::ProviderError;

async fn example() -> Result<String, ProviderError> {
    // ...
}
```

### Component Initialization Changes

**Before:**
```rust
use rumors::{RecipeConverter, RecipeChat};

let converter = RecipeConverter::new("api-key".to_string());
let chat = RecipeChat::new("api-key".to_string());
```

**After:**
```rust
use rumors::{RecipeConverter, RecipeChat, AiClient, ProviderType, create_provider};

let provider = create_provider(ProviderType::GoogleGemini {
    api_key: "api-key".to_string(),
});
let client = AiClient::new(provider);

let converter = RecipeConverter::new(client.clone());
let chat = RecipeChat::new(client);
```

## New Features

### Ollama Support

```rust
use rumors::{RumorsClient, ProviderType};

// Using Ollama
let client = RumorsClient::new(ProviderType::Ollama {
    base_url: "http://localhost:11434".to_string(),
    model: "llama2".to_string(),
});

// Convenience constructor
let client = RumorsClient::new_with_ollama(
    "http://localhost:11434".to_string(),
    "llama2".to_string(),
);
```

### Provider Abstraction

```rust
use rumors::{AiProvider, create_provider, ProviderType};

// Create providers programmatically
let provider = create_provider(provider_type);

// Use the provider directly if needed
let response = provider.generate_content("prompt").await?;
```

## Migration Steps

1. **Update imports**: Add `ProviderType` and `ProviderError` to your imports
2. **Update client creation**: Use the new constructor or legacy convenience method
3. **Update error handling**: Change `GoogleApiError` to `ProviderError`
4. **Test**: Run your existing code to ensure it works

## Backwards Compatibility

The library maintains backwards compatibility through:
- `RumorsClient::new_with_google_api()` convenience constructor
- `GoogleApiError` is still exported (re-exported as `ProviderError`)
- All existing method signatures remain the same (only error types changed)

## Dependencies

The library now requires:
- `async-trait = "0.1"` (new dependency)
- All existing dependencies remain the same