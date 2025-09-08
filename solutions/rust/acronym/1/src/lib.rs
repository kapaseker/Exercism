use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    String::from_iter(Regex::new(r"([a-z']+)|([A-Z][a-z']+)|([A-Z]+)").unwrap().find_iter(phrase).map(|s| s.as_str().chars().next().unwrap().to_ascii_uppercase()))
}
