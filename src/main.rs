use regex;
use std::{collections::HashSet, hash, ops};
use url::Url;

// const DATA_URL_DEFAULT_MIME_TYPE: &'static str = "text/plain";
// const DATA_URL_DEFAULT_CHARSET: &'static str = "us-ascii";

fn has_custom_protocol(url_string: &str) -> bool {
  let supported_protocols: HashSet<&str> = ["https", "http", "file"].iter().cloned().collect();

  match Url::parse(url_string) {
    Ok(url) => supported_protocols.contains(url.scheme()),
    Err(_) => false,
  }
}

#[derive(Debug)]
struct NormalizeOptions {
  strip_hash: bool,
}

impl Default for NormalizeOptions {
  fn default() -> Self {
    NormalizeOptions { strip_hash: false }
  }
}

fn normalize_data_url(url_string: &str, options: NormalizeOptions) -> String {
  todo!("normalize data url");
}

fn normalize_url(url_string: &str, options: &str) -> String {
  // println!("{:?}, {:?} test", url_string, options);
  let trimmed_url = url_string.trim().to_string();

  trimmed_url
}

fn main() {
  let url_string = "https://example.com";

  let options: &str = "test";
  // println!("{:?}", config);

  let result = normalize_url(url_string, options);

  println!("Result: {:?} ", result);

  // let result = has_custom_protocol(url_string);
  // println!("Does the URL have a custom protocol? {}", result);

  // print!("{:?}, {:?} test", url_string, options);
}
