use regex::Regex;
use lazy_static::lazy_static;

// Compiling a regex is expensive, so we do it once and reuse it everywhere
lazy_static! {
    static ref DOMAIN_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
}

pub fn is_valid_domain(domain: &str) -> bool {
    DOMAIN_REGEX.is_match(domain)
}

// pub fn is_valid_parameters(parameters: &[String]) -> bool {
//     !parameters.is_empty() && parameters.iter().all(|p| !p.trim().is_empty())
// }

