#[cfg(test)]
mod tests {
    use rumors::{AiClient, ProviderType, RecipeChat, RecipeConverter, RumorsClient, RumorsResponse, create_provider};

    fn get_test_api_key() -> Option<String> {
        std::env::var("GOOGLE_API_KEY").ok()
    }

    #[tokio::test]
    async fn test_recipe_converter_initialization() {
        let provider = create_provider(ProviderType::GoogleGemini {
            api_key: "test-key".to_string(),
        });
        let client = AiClient::new(provider);
        let _converter = RecipeConverter::new(client);
        // Test that it initializes without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_chat_initialization() {
        let provider = create_provider(ProviderType::GoogleGemini {
            api_key: "test-key".to_string(),
        });
        let client = AiClient::new(provider);
        let _chat = RecipeChat::new(client);
        // Test that it initializes without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_rumors_client_initialization() {
        let _client = RumorsClient::new_with_google_api("test-key".to_string());
        // Test that it initializes without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_rumors_client_initialization_new_api() {
        let _client = RumorsClient::new(ProviderType::GoogleGemini {
            api_key: "test-key".to_string(),
        });
        // Test that it initializes without panicking
        assert!(true);
    }

    #[tokio::test]
    async fn test_ollama_client_initialization() {
        let _client = RumorsClient::new_with_ollama(
            "http://localhost:11434".to_string(),
            "llama2".to_string(),
        );
        // Test that it initializes without panicking
        assert!(true);
    }

    #[tokio::test]
    #[ignore] // Requires actual API key
    async fn test_recipe_conversion_integration() {
        if let Some(api_key) = get_test_api_key() {
            let client = RumorsClient::new_with_google_api(api_key);

            let recipe = "Simple Pasta: 200g pasta, 100g cheese, olive oil, salt";

            match client.convert_recipe_to_shopping_list(recipe).await {
                Ok(shopping_list) => {
                    assert!(!shopping_list.items.is_empty());
                    println!("Shopping list generated successfully: {:#?}", shopping_list);
                }
                Err(e) => {
                    eprintln!("API call failed: {}", e);
                    panic!("Integration test failed");
                }
            }
        } else {
            println!("Skipping integration test - no API key provided");
        }
    }

    #[tokio::test]
    #[ignore] // Requires actual API key
    async fn test_chat_integration() {
        if let Some(api_key) = get_test_api_key() {
            let client = RumorsClient::new_with_google_api(api_key);

            match client
                .ask_cooking_question("What is pasta?".to_string())
                .await
            {
                Ok(response) => {
                    assert!(!response.is_empty());
                    println!("Chat response: {}", response);
                }
                Err(e) => {
                    eprintln!("Chat API call failed: {}", e);
                    panic!("Chat integration test failed");
                }
            }
        } else {
            println!("Skipping chat integration test - no API key provided");
        }
    }

    #[tokio::test]
    #[ignore] // Requires actual API key
    async fn test_extract_shopping_list_main_function() {
        if let Some(api_key) = get_test_api_key() {
            let client = RumorsClient::new_with_google_api(api_key);

            let recipe = "Simple Pasta: 200g pasta, 100g cheese, olive oil, salt";

            match client.extract_shopping_list(recipe).await {
                Ok(shopping_list) => {
                    assert!(!shopping_list.items.is_empty());
                    println!("Shopping list (main function): {:#?}", shopping_list);
                }
                Err(e) => {
                    eprintln!("Main shopping list function failed: {}", e);
                    panic!("Main shopping list integration test failed");
                }
            }
        } else {
            println!("Skipping main shopping list integration test - no API key provided");
        }
    }

    #[tokio::test]
    #[ignore] // Requires actual API key
    async fn test_process_request_main_function() {
        if let Some(api_key) = get_test_api_key() {
            let client = RumorsClient::new_with_google_api(api_key);

            // Test general cooking question
            match client.process_request("What is the best way to cook pasta?", None).await {
                Ok(RumorsResponse::TextResponse(response)) => {
                    assert!(!response.is_empty());
                    println!("Process request (general): {}", response);
                }
                Ok(RumorsResponse::ShoppingList(_)) => {
                    panic!("Expected text response, got shopping list");
                }
                Err(e) => {
                    eprintln!("Process request failed: {}", e);
                    panic!("Process request integration test failed");
                }
            }

            // Test recipe analysis
            let recipe = "Simple Pasta: 200g pasta, 100g cheese, olive oil, salt, garlic";
            match client.process_request("Analyze this recipe", Some(recipe)).await {
                Ok(RumorsResponse::TextResponse(response)) => {
                    assert!(!response.is_empty());
                    println!("Process request (analysis): {}", response);
                }
                Ok(RumorsResponse::ShoppingList(_)) => {
                    panic!("Expected text response, got shopping list");
                }
                Err(e) => {
                    eprintln!("Process request analysis failed: {}", e);
                    panic!("Process request analysis integration test failed");
                }
            }
        } else {
            println!("Skipping process request integration test - no API key provided");
        }
    }
}
