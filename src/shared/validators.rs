use lazy_static::lazy_static;
use regex::Regex;

pub fn validate_slug(slug: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9\-\_]+$").unwrap();
    }
    RE.is_match(slug)
}
