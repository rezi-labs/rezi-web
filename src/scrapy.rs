use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use std::collections::HashSet;

lazy_static::lazy_static! {
    static ref BULLET_REGEX: Regex = Regex::new(r"^[\d\s]*[•\-\*]\s*").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"^\d+\.\s*").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
    static ref INGREDIENT_REGEX: Regex = Regex::new(r#""recipeIngredient"\s*:\s*\[(.*?)\]"#).unwrap();
    static ref ITEM_REGEX: Regex = Regex::new(r#""([^"]+)""#).unwrap();
    static ref MEASUREMENT_REGEX: Regex = Regex::new(
        r"(?i)(?:^|\n)\s*(?:\d+(?:\.\d+)?|\d+/\d+|\d+\s+\d+/\d+)?\s*(?:cups?|tbsp|tsp|teaspoons?|tablespoons?|oz|ounces?|lbs?|pounds?|g|grams?|kg|ml|l|liters?|cloves?|pieces?|slices?|strips?|dashes?|pinches?)\s+(?:of\s+)?([^\n\r]+)"
    ).unwrap();
}

pub fn extract_ingredients(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let mut ingredients = Vec::new();

    // Common CSS selectors for ingredients
    let ingredient_selectors = vec![
        "[itemprop='recipeIngredient']",
        ".recipe-ingredient",
        ".ingredient",
        ".ingredients li",
        ".recipe-ingredients li",
        "[class*='ingredient']",
        "[data-ingredient]",
        ".entry-ingredients li",
        ".ingredients-list li",
        ".recipe-card-ingredients li",
    ];

    // Try structured data selectors first
    for selector_str in &ingredient_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                let text = clean_ingredient_text(&element);
                if !text.is_empty() && is_likely_ingredient(&text) {
                    ingredients.push(text);
                }
            }
        }
    }

    // If no structured ingredients found, try JSON-LD
    if ingredients.is_empty() {
        ingredients.extend(extract_from_json_ld(&document));
    }

    // If still no ingredients, try fallback text patterns
    if ingredients.is_empty() {
        ingredients.extend(extract_from_text_patterns(html));
    }

    // Remove duplicates while preserving order
    let mut seen = HashSet::new();
    ingredients.retain(|item| seen.insert(item.clone()));

    ingredients
}

pub fn extract_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    
    // Common CSS selectors for recipe titles
    let title_selectors = vec![
        "h1[itemprop='name']",
        "[itemprop='name']",
        "h1.recipe-title",
        "h1.entry-title", 
        ".recipe-header h1",
        ".recipe-title",
        ".entry-title",
        "h1.post-title",
        "h1",
        "title",
        "[class*='recipe'][class*='title']",
        "[class*='title'][class*='recipe']",
    ];

    // Try structured data selectors first
    for selector_str in &title_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                let text = clean_title_text(&element);
                if !text.is_empty() && is_likely_title(&text) {
                    return Some(text);
                }
            }
        }
    }

    // If no title found, try JSON-LD structured data
    if let Some(title) = extract_title_from_json_ld(&document) {
        return Some(title);
    }

    None
}

fn clean_title_text(element: &ElementRef) -> String {
    let text = element.text().collect::<String>();
    
    // Clean up the title text
    let text = text.trim().replace(['\n', '\t'], " ");
    
    // Normalize whitespace
    WHITESPACE_REGEX.replace_all(&text, " ").trim().to_string()
}

fn is_likely_title(text: &str) -> bool {
    let len = text.len();
    
    // Reasonable title length
    if !(5..=150).contains(&len) {
        return false;
    }
    
    // Should not be just numbers or symbols
    if !text.chars().any(|c| c.is_alphabetic()) {
        return false;
    }
    
    // Ignore common non-title patterns
    let lowercase = text.to_lowercase();
    let ignore_patterns = vec![
        "recipe",
        "ingredients",
        "directions", 
        "instructions",
        "method",
        "home",
        "page",
        "website",
        "blog",
        "menu",
        "search",
        "login",
        "register",
    ];
    
    // If it's only one of these words, it's probably not a recipe title
    for pattern in ignore_patterns {
        if lowercase.trim() == pattern {
            return false;
        }
    }
    
    true
}

fn extract_title_from_json_ld(document: &Html) -> Option<String> {
    let script_selector = Selector::parse("script[type='application/ld+json']").unwrap();
    
    for script in document.select(&script_selector) {
        let json_text = script.text().collect::<String>();
        
        // Simple regex-based extraction for recipe name
        if let Some(captures) = Regex::new(r#""name"\s*:\s*"([^"]+)""#).unwrap().captures(&json_text) {
            if let Some(title) = captures.get(1) {
                let title = title.as_str().trim().to_string();
                if is_likely_title(&title) {
                    return Some(title);
                }
            }
        }
    }
    
    None
}

fn clean_ingredient_text(element: &ElementRef) -> String {
    // Get text content and clean it
    let text = element.text().collect::<String>();

    // Remove extra whitespace and common prefixes/suffixes
    let text = text.trim().replace(['\n', '\t'], " ");

    // Remove bullet points and numbering
    let text = BULLET_REGEX.replace(&text, "");

    // Remove leading numbers (1., 2., etc.)
    let text = NUMBER_REGEX.replace(&text, "");

    // Normalize whitespace
    WHITESPACE_REGEX.replace_all(&text, " ").trim().to_string()
}

fn is_likely_ingredient(text: &str) -> bool {
    // Basic filters for ingredient-like text
    let len = text.len();

    // Too short or too long
    if !(3..=200).contains(&len) {
        return false;
    }

    // Ignore common non-ingredient text
    let lowercase = text.to_lowercase();
    let ignore_patterns = vec![
        "ingredients",
        "directions",
        "instructions",
        "method",
        "preparation",
        "serves",
        "cooking time",
        "prep time",
        "difficulty",
        "recipe",
        "print",
        "share",
        "save",
        "rating",
        "review",
        "comment",
    ];

    for pattern in ignore_patterns {
        if lowercase.contains(pattern) {
            return false;
        }
    }

    // Must contain at least one letter
    text.chars().any(|c| c.is_alphabetic())
}

fn extract_from_json_ld(document: &Html) -> Vec<String> {
    let mut ingredients = Vec::new();

    // Look for JSON-LD structured data
    let script_selector = Selector::parse("script[type='application/ld+json']").unwrap();

    for script in document.select(&script_selector) {
        let json_text = script.text().collect::<String>();

        // Simple regex-based extraction for recipeIngredient
        if let Some(captures) = INGREDIENT_REGEX.captures(&json_text) {
            let ingredients_str = captures.get(1).unwrap().as_str();

            // Extract individual ingredients from the JSON array
            for cap in ITEM_REGEX.captures_iter(ingredients_str) {
                let ingredient = cap.get(1).unwrap().as_str().to_string();
                if is_likely_ingredient(&ingredient) {
                    ingredients.push(ingredient);
                }
            }
        }
    }

    ingredients
}

fn extract_from_text_patterns(html: &str) -> Vec<String> {
    let mut ingredients = Vec::new();

    // Look for common measurement patterns that indicate ingredients
    for cap in MEASUREMENT_REGEX.captures_iter(html) {
        if let Some(ingredient_match) = cap.get(1) {
            let ingredient = ingredient_match.as_str().trim();
            if is_likely_ingredient(ingredient) {
                ingredients.push(ingredient.to_string());
            }
        }
    }

    ingredients
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ingredients_structured() {
        let html = r#"
            <div class="recipe">
                <li itemprop="recipeIngredient">2 cups flour</li>
                <li itemprop="recipeIngredient">1 tsp salt</li>
                <li itemprop="recipeIngredient">3 eggs</li>
            </div>
        "#;

        let ingredients = extract_ingredients(html);
        assert_eq!(ingredients.len(), 3);
        assert!(ingredients.contains(&"2 cups flour".to_string()));
        assert!(ingredients.contains(&"1 tsp salt".to_string()));
        assert!(ingredients.contains(&"3 eggs".to_string()));
    }

    #[test]
    fn test_extract_ingredients_json_ld() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@type": "Recipe",
                "recipeIngredient": ["2 cups flour", "1 tsp salt", "3 eggs"]
            }
            </script>
        "#;

        let ingredients = extract_ingredients(html);
        assert_eq!(ingredients.len(), 3);
        assert!(ingredients.contains(&"2 cups flour".to_string()));
    }

    #[test]
    fn test_extract_title_structured() {
        let html = r#"
            <div class="recipe">
                <h1 itemprop="name">Chocolate Chip Cookies</h1>
                <li itemprop="recipeIngredient">2 cups flour</li>
            </div>
        "#;

        let title = extract_title(html);
        assert_eq!(title, Some("Chocolate Chip Cookies".to_string()));
    }

    #[test]
    fn test_extract_title_json_ld() {
        let html = r#"
            <script type="application/ld+json">
            {
                "@type": "Recipe",
                "name": "Best Banana Bread",
                "recipeIngredient": ["3 bananas", "2 cups flour"]
            }
            </script>
        "#;

        let title = extract_title(html);
        assert_eq!(title, Some("Best Banana Bread".to_string()));
    }

    #[test]
    fn test_extract_title_h1_fallback() {
        let html = r#"
            <html>
                <head><title>Amazing Pancakes Recipe - Food Blog</title></head>
                <body>
                    <h1>Amazing Pancakes</h1>
                    <p>These are great pancakes</p>
                </body>
            </html>
        "#;

        let title = extract_title(html);
        assert_eq!(title, Some("Amazing Pancakes".to_string()));
    }
}
