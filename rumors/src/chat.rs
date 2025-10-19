use crate::client::AiClient;
use crate::providers::ProviderError;

#[derive(Debug, Clone)]
pub struct ChatSession {
    client: AiClient,
    system_prompt: Option<String>,
}

impl ChatSession {
    pub fn new(client: AiClient) -> Self {
        Self {
            client,
            system_prompt: None,
        }
    }

    pub fn with_system_prompt(mut self, system_prompt: String) -> Self {
        self.system_prompt = Some(system_prompt);
        self
    }

    pub async fn send_message(&self, user_message: String) -> Result<String, ProviderError> {
        let context = self.build_context(&user_message);
        self.client.generate_content(&context).await
    }

    fn build_context(&self, user_message: &str) -> String {
        let mut context = String::new();

        if let Some(system_prompt) = &self.system_prompt {
            context.push_str(&format!("System: {system_prompt}\n\n"));
        }

        context.push_str(&format!("User: {user_message}"));
        context
    }
}

pub struct RecipeChat {
    session: ChatSession,
}

impl RecipeChat {
    pub fn new(client: AiClient) -> Self {
        let system_prompt = r#"You are a helpful cooking assistant. You can help users with:
1. Converting recipes to shopping lists
2. Answering cooking questions
3. Suggesting recipe modifications
4. Providing cooking tips and techniques

When asked to convert a recipe to a shopping list, provide a clear, organized list with quantities and categories."#;

        let session = ChatSession::new(client)
            .with_system_prompt(system_prompt.to_string());

        Self { session }
    }

    pub async fn ask(&self, question: String) -> Result<String, ProviderError> {
        self.session.send_message(question).await
    }

    pub async fn discuss_recipe(&self, recipe_text: &str, question: String) -> Result<String, ProviderError> {
        let formatted_question = format!(
            r#"Here's a recipe I'd like to discuss:

Recipe:
{recipe_text}

Question: {question}"#
        );
        
        self.session.send_message(formatted_question).await
    }

    pub async fn analyze_recipe(&self, recipe_text: &str) -> Result<String, ProviderError> {
        let analysis_prompt = format!(
            r#"Please analyze this recipe and provide insights about:
1. Difficulty level and estimated cooking time
2. Key techniques involved
3. Potential substitutions for ingredients
4. Tips for success
5. Nutritional highlights
6. Possible variations

Recipe:
{recipe_text}"#
        );
        
        self.session.send_message(analysis_prompt).await
    }

    pub async fn suggest_recipe_modifications(&self, recipe_text: &str, dietary_requirements: &str) -> Result<String, ProviderError> {
        let modification_prompt = format!(
            r#"Please suggest modifications to make this recipe suitable for: {dietary_requirements}

Original Recipe:
{recipe_text}

Please provide:
1. Specific ingredient substitutions
2. Cooking method adjustments
3. Any timing changes needed
4. Expected changes in taste/texture"#
        );
        
        self.session.send_message(modification_prompt).await
    }
}
