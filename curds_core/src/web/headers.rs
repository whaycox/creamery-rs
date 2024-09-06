use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::OnceLock;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct HttpHeaders {
    pub host: Option<String>,
    pub user_agent: Option<String>,
    pub connection: Option<Vec<String>>,
    pub referrer: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<usize>,
    pub accept: Option<Vec<String>>,
    pub accept_encoding: Option<Vec<String>>,
    pub accept_language: Option<Vec<String>>,

    pub general: HashMap<String, Vec<String>>,
}

const HOST_PATTERN: &str = r"^(?i)Host$";
static HOST_REGEX: OnceLock<Regex> = OnceLock::new();
fn host_match() -> &'static Regex { HOST_REGEX.get_or_init(|| Regex::new(HOST_PATTERN).unwrap()) }

const USER_AGENT_PATTERN: &str = r"^(?i)User-Agent$";
static USER_AGENT_REGEX: OnceLock<Regex> = OnceLock::new();
fn user_agent_match() -> &'static Regex { USER_AGENT_REGEX.get_or_init(|| Regex::new(USER_AGENT_PATTERN).unwrap()) }

const CONNECTION_PATTERN: &str = r"^(?i)Connection$";
static CONNECTION_REGEX: OnceLock<Regex> = OnceLock::new();
fn connection_match() -> &'static Regex { CONNECTION_REGEX.get_or_init(|| Regex::new(CONNECTION_PATTERN).unwrap()) }

const REFERRER_PATTERN: &str = r"^(?i)Referer$";
static REFERRER_REGEX: OnceLock<Regex> = OnceLock::new();
fn referrer_match() -> &'static Regex { REFERRER_REGEX.get_or_init(|| Regex::new(REFERRER_PATTERN).unwrap()) }

const CONTENT_TYPE_PATTERN: &str = r"^(?i)Content-Type$";
static CONTENT_TYPE_REGEX: OnceLock<Regex> = OnceLock::new();
fn content_type_match() -> &'static Regex { CONTENT_TYPE_REGEX.get_or_init(|| Regex::new(CONTENT_TYPE_PATTERN).unwrap()) }

const CONTENT_LENGTH_PATTERN: &str = r"^(?i)Content-Length$";
static CONTENT_LENGTH_REGEX: OnceLock<Regex> = OnceLock::new();
fn content_length_match() -> &'static Regex { CONTENT_LENGTH_REGEX.get_or_init(|| Regex::new(CONTENT_LENGTH_PATTERN).unwrap()) }

const ACCEPT_PATTERN: &str = r"^(?i)Accept$";
static ACCEPT_REGEX: OnceLock<Regex> = OnceLock::new();
fn accept_match() -> &'static Regex { ACCEPT_REGEX.get_or_init(|| Regex::new(ACCEPT_PATTERN).unwrap()) }

const ACCEPT_ENCODING_PATTERN: &str = r"^(?i)Accept-Encoding$";
static ACCEPT_ENCODING_REGEX: OnceLock<Regex> = OnceLock::new();
fn accept_encoding_match() -> &'static Regex { ACCEPT_ENCODING_REGEX.get_or_init(|| Regex::new(ACCEPT_ENCODING_PATTERN).unwrap()) }

const ACCEPT_LANGUAGE_PATTERN: &str = r"^(?i)Accept-Language$";
static ACCEPT_LANGUAGE_REGEX: OnceLock<Regex> = OnceLock::new();
fn accept_language_match() -> &'static Regex { ACCEPT_LANGUAGE_REGEX.get_or_init(|| Regex::new(ACCEPT_LANGUAGE_PATTERN).unwrap()) }

const CSV_PATTERN: &str = r",\s*";
static CSV_REGEX: OnceLock<Regex> = OnceLock::new();
fn csv_match() -> &'static Regex { CSV_REGEX.get_or_init(|| Regex::new(CSV_PATTERN).unwrap()) }

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            host: None,
            user_agent: None,
            connection: None,
            referrer: None,
            content_type: None,
            content_length: None,
            accept: None,
            accept_encoding: None,
            accept_language: None,
            general: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: String, value: String) {
        if host_match().is_match(&key) { self.host = Some(value); }
        else if user_agent_match().is_match(&key) { self.user_agent = Some(value); }
        else if connection_match().is_match(&key) { Self::extend_split_values(&mut self.connection, value); }
        else if referrer_match().is_match(&key) { self.referrer = Some(value); }
        else if content_type_match().is_match(&key) { self.content_type = Some(value); }
        else if content_length_match().is_match(&key) { self.content_length = Some(value.parse::<usize>().unwrap()); }
        else if accept_match().is_match(&key) { Self::extend_split_values(&mut self.accept, value); }
        else if accept_encoding_match().is_match(&key) { Self::extend_split_values(&mut self.accept_encoding, value); }
        else if accept_language_match().is_match(&key) { Self::extend_split_values(&mut self.accept_language, value); }
        else {
            let split_values = Self::split_comma_separated_values(value);
            match self.general.entry(key.to_lowercase()) {
                Entry::Vacant(vacant) => { vacant.insert(split_values); },
                Entry::Occupied(mut occupied) => { occupied.get_mut().extend(split_values); },
            }
        }
    }
    fn extend_split_values(header: &mut Option<Vec<String>>, value: String) {
        let split_values = Self::split_comma_separated_values(value);
        match header {
            Some(existing) => existing.extend(split_values),
            None => { *header = Some(split_values); },
        }
    }
    fn split_comma_separated_values(mut value: String) -> Vec<String> {
        let mut matches: Vec<(usize, usize)> = csv_match()
            .find_iter(&value)
            .map(|comma_separator| (comma_separator.start(), comma_separator.len()))
            .collect();
        matches.reverse();
        let mut values: Vec<String> = Vec::new();
        for (start, length) in matches {
            let tail = value.split_off(start + length);
            if tail.len() > 0 {
                values.push(tail);
            }
            value.truncate(start);
        }
        if value.len() > 0 {
            values.push(value);
        }
        values.reverse();

        values
    }
}