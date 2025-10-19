use rumors::{ProviderType, RumorsClient};

#[tokio::test]
async fn test_rumors_client_creation() {
    // Test Google Gemini provider creation
    let _client_gemini = RumorsClient::new(ProviderType::GoogleGemini {
        api_key: "test_key".to_string(),
    });

    // Test Ollama provider creation
    let _client_ollama = RumorsClient::new(ProviderType::Ollama {
        base_url: "http://localhost:11434".to_string(),
        model: "llama2".to_string(),
    });

    // Test legacy constructor
    let _client_legacy = RumorsClient::new_with_google_api("test_key".to_string());

    // Test new Ollama constructor
    let _client_ollama_new = RumorsClient::new_with_ollama(
        "http://localhost:11434".to_string(),
        "llama2".to_string(),
    );
}

#[tokio::test]
async fn test_provider_types() {
    let gemini_provider = ProviderType::GoogleGemini {
        api_key: "test".to_string(),
    };
    
    let ollama_provider = ProviderType::Ollama {
        base_url: "http://localhost:11434".to_string(),
        model: "llama2".to_string(),
    };

    match gemini_provider {
        ProviderType::GoogleGemini { api_key } => {
            assert_eq!(api_key, "test");
        }
        _ => panic!("Wrong provider type"),
    }

    match ollama_provider {
        ProviderType::Ollama { base_url, model } => {
            assert_eq!(base_url, "http://localhost:11434");
            assert_eq!(model, "llama2");
        }
        _ => panic!("Wrong provider type"),
    }
}