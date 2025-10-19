# Rumors - Recipe to Shopping List Converter

A Rust library that supports multiple AI providers (Google Gemini API and Ollama) to convert recipes into shopping lists and provides intelligent cooking assistance through AI-powered chat functionality.

## Features

- **Multiple AI Provider Support**: Works with Google Gemini API and Ollama
- Convert recipe text into structured shopping lists
- Intelligent request processing with automatic function routing
- Interactive chat interface for cooking questions
- Recipe analysis and discussion
- Recipe modification suggestions
- Stateless design for easy integration
- Easy provider switching for development and production

## Usage

### Provider Selection

Choose between Google Gemini API and Ollama:

```rust
use rumors::{RumorsClient, ProviderType};

// Option 1: Google Gemini API
let client = RumorsClient::new(ProviderType::GoogleGemini {
    api_key: "your-google-api-key".to_string(),
});

// Option 2: Ollama
let client = RumorsClient::new(ProviderType::Ollama {
    base_url: "http://localhost:11434".to_string(),
    model: "llama2".to_string(),
});

// Convenience methods
let client = RumorsClient::new_with_google_api("your-api-key".to_string());
let client = RumorsClient::new_with_ollama("http://localhost:11434".to_string(), "llama2".to_string());
```

### Main Functions

The library provides two primary entry points:

#### 1. Direct Shopping List Extraction

```rust
use rumors::{RumorsClient, ProviderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using Google Gemini API
    let client = RumorsClient::new(ProviderType::GoogleGemini {
        api_key: "your-google-api-key".to_string(),
    });
    
    let recipe = "
        Spaghetti Carbonara
        
        Ingredients:
        - 400g spaghetti
        - 200g pancetta
        - 4 large eggs
        - 100g Pecorino Romano cheese
        - Black pepper
        - Salt
    ";
    
    // Main Function 1: Direct shopping list extraction
    let shopping_list = client.extract_shopping_list(recipe).await?;
    println!("Shopping List: {:#?}", shopping_list);
    
    Ok(())
}
```

#### 2. Intelligent Request Processing

```rust
use rumors::{RumorsClient, RumorsResponse, ProviderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Using Ollama for local processing
    let client = RumorsClient::new(ProviderType::Ollama {
        base_url: "http://localhost:11434".to_string(),
        model: "llama2".to_string(),
    });
    
    let recipe = "Your recipe text here...";
    
    // Main Function 2: Intelligent request processing
    // The AI automatically determines which function to use based on the request
    
    // General cooking question (no recipe needed)
    let response = client.process_request(
        "How do I prevent carbonara from becoming scrambled eggs?",
        None
    ).await?;
    
    // Recipe-specific question
    let response = client.process_request(
        "What wine pairs well with this dish?",
        Some(recipe)
    ).await?;
    
    // Recipe analysis request
    let response = client.process_request(
        "Can you analyze this recipe for me?",
        Some(recipe)
    ).await?;
    
    // Recipe modification request
    let response = client.process_request(
        "Make this recipe vegetarian",
        Some(recipe)
    ).await?;
    
    match response {
        RumorsResponse::ShoppingList(list) => {
            println!("Shopping List: {:#?}", list);
        }
        RumorsResponse::TextResponse(text) => {
            println!("Response: {}", text);
        }
    }
    
    Ok(())
}
```

### Advanced Usage - Direct Function Access

If you need more control, you can access individual functions directly:

```rust
use rumors::{RumorsClient, ProviderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RumorsClient::new_with_google_api("your-api-key".to_string());
    let recipe = "Your recipe text...";
    
    // Direct function access
    let shopping_list = client.convert_recipe_to_shopping_list(recipe).await?;
    let ingredients = client.extract_ingredients(recipe).await?;
    let answer = client.ask_cooking_question("How do I dice onions?".to_string()).await?;
    let discussion = client.discuss_recipe(recipe, "What's the origin of this dish?".to_string()).await?;
    let analysis = client.analyze_recipe(recipe).await?;
    let modifications = client.suggest_recipe_modifications(recipe, "gluten-free").await?;
    
    Ok(())
}
```

### Individual Components

You can also use the underlying components directly:

```rust
use rumors::{RecipeConverter, RecipeChat, AiClient, ProviderType, create_provider};

// Create a provider and client
let provider = create_provider(ProviderType::GoogleGemini {
    api_key: "your-api-key".to_string(),
});
let client = AiClient::new(provider);

// Recipe converter
let converter = RecipeConverter::new(client.clone());
let shopping_list = converter.convert_recipe_to_shopping_list("recipe text").await?;
let ingredients = converter.extract_ingredients_only("recipe text").await?;

// Chat interface
let chat = RecipeChat::new(client);
let response = chat.ask("What temperature should I cook chicken?".to_string()).await?;
```

## API Reference

### Main Functions

#### `extract_shopping_list(recipe_text: &str) -> Result<ShoppingList, ProviderError>`
Dedicated function for converting recipe text into a structured shopping list.

#### `process_request(input: &str, recipe_text: Option<&str>) -> Result<RumorsResponse, ProviderError>`
Intelligent router that uses AI to classify user intent and automatically calls the appropriate function:
- **General cooking questions**: Routes to cooking Q&A
- **Recipe discussions**: Routes to recipe-specific chat
- **Recipe analysis**: Routes to detailed recipe analysis  
- **Recipe modifications**: Routes to modification suggestions

### Direct Access Methods

- `convert_recipe_to_shopping_list(recipe_text)` - Convert recipe to structured shopping list
- `extract_ingredients(recipe_text)` - Extract ingredient names only
- `ask_cooking_question(question)` - Ask general cooking questions
- `discuss_recipe(recipe_text, question)` - Ask questions about a specific recipe
- `analyze_recipe(recipe_text)` - Get detailed recipe analysis
- `suggest_recipe_modifications(recipe_text, dietary_requirements)` - Get recipe modifications

## Provider Setup

### Google Gemini API

You need a Google AI API key to use this library. Get one from:
https://makersuite.google.com/app/apikey

Set it as an environment variable or pass it directly to the client.

### Ollama

For Ollama, you need to have Ollama running locally or on a remote server. Install Ollama from:
https://ollama.ai/

Pull a model (e.g., `ollama pull llama2`) and ensure the service is running on the specified URL (default: http://localhost:11434).

## Data Structures

### RumorsResponse
```rust
pub enum RumorsResponse {
    ShoppingList(ShoppingList),
    TextResponse(String),
}
```

### ShoppingList
```rust
pub struct ShoppingList {
    pub items: Vec<ShoppingItem>,
    pub recipe_title: Option<String>,
}

pub struct ShoppingItem {
    pub name: String,
    pub quantity: Option<String>,
    pub category: Option<String>,
}
```

## How It Works

### Intelligent Request Processing

The `process_request()` function uses AI to automatically determine user intent:

1. **Intent Classification**: Analyzes the user's input to classify it into one of four categories:
   - General cooking questions
   - Recipe-specific discussions
   - Recipe analysis requests
   - Recipe modification requests

2. **Automatic Routing**: Based on the classification, routes the request to the appropriate specialized function

3. **Context Awareness**: Takes into account whether a recipe is provided to make better routing decisions

### Stateless Design

The library uses a stateless architecture:
- No conversation history is maintained
- Each request is processed independently
- Easy to integrate into web services and APIs
- Thread-safe and scalable

## Examples

### Recipe Analysis
```rust
let response = client.process_request(
    "What's the nutritional profile of this recipe?",
    Some(recipe_text)
).await?;
```

### Dietary Modifications
```rust
let response = client.process_request(
    "Make this recipe vegan and gluten-free",
    Some(recipe_text)
).await?;
```

### Cooking Tips
```rust
let response = client.process_request(
    "What's the best way to caramelize onions?",
    None
).await?;
```