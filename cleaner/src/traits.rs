/// Trait for URL cleaners that determine which query parameters should be removed
pub trait UrlCleaner {
    fn should_remove(&self, key: &str) -> bool;
}