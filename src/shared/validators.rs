use lazy_static::lazy_static;
use regex::Regex;

pub fn validate_slug(slug: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-z0-9\-]+$").unwrap();
    }
    RE.is_match(slug)
}

pub fn validate_name(slug: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\w+(\s+\w+)*$").unwrap();
    }
    RE.is_match(slug)
}
