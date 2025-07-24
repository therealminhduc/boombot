use crate::registry::get_cleaner_for_host_string;
use crate::config::load_registry_with_fallback;
use crate::traits::UrlCleaner;

use url::Url;

/// Removes tracking parameters from the URL
pub fn clean_url(input: &str) -> Result<String, url::ParseError> {
    let mut url = Url::parse(input)?;
    let host = url.host_str().unwrap_or("");

    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "../cleaner/rules.db".to_string());
    let registry = load_registry_with_fallback(&db_path)
        .expect("Failed to load domain rules config file");

    let cleaner = get_cleaner_for_host_string(host, &registry);

    let cleaned_pairs: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| !cleaner.should_remove(key))
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();

    url.set_query(None);

    if !cleaned_pairs.is_empty() {
        let mut query = url.query_pairs_mut();
        for (key, value) in cleaned_pairs {
            query.append_pair(&key, &value);
        }
    }

    Ok(url.to_string())
}



/* =============================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_url_with_query_params() {
        let input = "https://example.com?utm_source=test&param=keep";
        let result = clean_url(input).unwrap();
        
        assert!(result.contains("param=keep"));
        assert!(!result.contains("utm_source"));
    }

    #[test]
    fn handles_domain_specific_cleaning() {
        let input = "https://instagram.com?igsh=123&utm_source=test&param=keep";
        let result = clean_url(input).unwrap();
        
        assert!(result.contains("param=keep"));
        assert!(!result.contains("igsh"));
        assert!(!result.contains("utm_source"));
    }

    #[test]
    fn preserves_url_structure() {
        let input = "https://example.com/path?param=keep";
        let result = clean_url(input).unwrap();
        
        assert!(result.starts_with("https://example.com/"));
        assert!(result.contains("param=keep"));
    }
}