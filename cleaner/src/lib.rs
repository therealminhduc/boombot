pub mod traits;
pub mod cleaners;
pub mod registry;
pub mod config;
pub mod engine;

pub use engine::clean_url;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_utm_parameters() {
        let input = "https://example.com?utm_source=test&utm_medium=email&utm_campaign=newsletter&param=value";
        let expected = "https://example.com/?param=value";
        assert_eq!(clean_url(input).unwrap(), expected);
    }

    #[test]
    fn removes_instagram_tracking() {
        let input = "https://instagram.com?igsh=123&utm_source=test&param=value";
        let expected = "https://instagram.com/?param=value";
        assert_eq!(clean_url(input).unwrap(), expected);
    }

    #[test]
    fn removes_linkedin_tracking() {
        let input = "https://linkedin.com?rcm=123&utm_campaign=test&param=value";
        let expected = "https://linkedin.com/?param=value";
        assert_eq!(clean_url(input).unwrap(), expected);
    }

    #[test]
    fn keeps_url_without_tracking_params() {
        let input = "https://example.com?param1=value1&param2=value2";
        let expected = "https://example.com/?param1=value1&param2=value2";
        assert_eq!(clean_url(input).unwrap(), expected);
    }

    #[test]
    fn handles_url_without_query() {
        let input = "https://example.com";
        let expected = "https://example.com/";
        assert_eq!(clean_url(input).unwrap(), expected);
    }

    #[test]
    fn handles_invalid_url() {
        let input = "Not a url";
        assert!(clean_url(input).is_err());
    }
}
