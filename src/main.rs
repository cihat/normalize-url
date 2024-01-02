use std::collections::HashSet;
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

fn main() {
  let url_string = "https://example.com";
  let result = has_custom_protocol(url_string);
  println!("Does the URL have a custom protocol? {}", result);
}
