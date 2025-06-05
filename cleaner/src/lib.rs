use url::Url;


/// Removes tracking parameters from the URL
pub fn clean_url(input: &str) -> Result<String, url::ParseError> {
    let mut url = Url::parse(input)?;

    let cleaned_pairs: Vec<(String, String)> = url.query_pairs()
        .filter( |(key, _)| {
            !key.starts_with("utm_") && key != "fbclid" && key != "igshid"
        })
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
    fn removes_fbclid_and_igshid() {
        let input = "https://example.com?fbclid=123&igshid=456&param=value";
        let expected = "https://example.com/?param=value";

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
