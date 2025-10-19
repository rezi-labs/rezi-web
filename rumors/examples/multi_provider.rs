use rumors::{ProviderType, RumorsClient, RumorsResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using Google Gemini API
    println!("=== Google Gemini API Example ===");
    if let Ok(api_key) = std::env::var("GOOGLE_API_KEY") {
        let client = RumorsClient::new(ProviderType::GoogleGemini { api_key });
        
        let recipe = r#"
            Spaghetti Carbonara
            
            Ingredients:
            - 400g spaghetti
            - 200g pancetta
            - 4 large eggs
            - 100g Pecorino Romano cheese
            - Black pepper
            - Salt
        "#;
        
        // Test shopping list extraction
        match client.extract_shopping_list(recipe).await {
            Ok(shopping_list) => {
                println!("Shopping List: {:#?}", shopping_list);
            }
            Err(e) => println!("Error: {}", e),
        }
        
        // Test intelligent request processing
        match client.process_request("What wine pairs well with carbonara?", Some(recipe)).await {
            Ok(RumorsResponse::TextResponse(response)) => {
                println!("Wine pairing suggestion: {}", response);
            }
            Ok(RumorsResponse::ShoppingList(_)) => {
                println!("Unexpected shopping list response");
            }
            Err(e) => println!("Error: {}", e),
        }
    } else {
        println!("GOOGLE_API_KEY not set, skipping Gemini example");
    }
    
    // Example 2: Using Ollama (assuming it's running locally)
    println!("\n=== Ollama Example ===");
    let client = RumorsClient::new(ProviderType::Ollama {
        base_url: "http://localhost:11434".to_string(),
        model: "llama2".to_string(),
    });
    
    // Test with a simple cooking question
    match client.ask_cooking_question("How do I make scrambled eggs?".to_string()).await {
        Ok(response) => {
            println!("Ollama response: {}", response);
        }
        Err(e) => {
            println!("Ollama error (is Ollama running?): {}", e);
        }
    }
    
    // Example 3: Using convenience constructors
    println!("\n=== Convenience Constructors ===");
    
    // Legacy Google API constructor (for backwards compatibility)
    if let Ok(api_key) = std::env::var("GOOGLE_API_KEY") {
        let _client = RumorsClient::new_with_google_api(api_key);
        println!("Google client created with convenience constructor");
    }
    
    // Ollama convenience constructor
    let _client = RumorsClient::new_with_ollama(
        "http://localhost:11434".to_string(),
        "llama2".to_string(),
    );
    println!("Ollama client created with convenience constructor");
    
    Ok(())
}