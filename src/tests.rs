use super::*;

// -- extract_title_from_html --

#[test]
fn test_extract_title_valid_title() {
    let html = "<html><head><title>Hello World</title></head><body></body></html>";
    assert_eq!(extract_title_from_html(html), "Hello World");
}

#[test]
fn test_extract_title_empty_title_fallback_to_h1() {
    let html = "<html><head><title></title></head><body><h1>Fallback Header</h1></body></html>";
    assert_eq!(extract_title_from_html(html), "Fallback Header (H1 tag)");
}

#[test]
fn test_extract_title_whitespace_title_fallback_to_h1() {
    let html =
        "<html><head><title>   \n\t  </title></head><body><h1>Real Header</h1></body></html>";
    assert_eq!(extract_title_from_html(html), "Real Header (H1 tag)");
}

#[test]
fn test_extract_title_no_title_with_h1() {
    let html = "<html><body><h1>Only H1 Here</h1></body></html>";
    assert_eq!(extract_title_from_html(html), "Only H1 Here (H1 tag)");
}

#[test]
fn test_extract_title_no_title_empty_h1() {
    let html = "<html><body><h1></h1></body></html>";
    assert_eq!(extract_title_from_html(html), "No title found");
}

#[test]
fn test_extract_title_no_title_no_h1() {
    let html = "<html><body><p>Just a paragraph</p></body></html>";
    assert_eq!(extract_title_from_html(html), "No title found");
}

#[test]
fn test_extract_title_trims_whitespace() {
    let html = "<html><head><title>  Spaced Title  </title></head></html>";
    assert_eq!(extract_title_from_html(html), "Spaced Title");
}

#[test]
fn test_extract_title_h1_trims_whitespace() {
    let html = "<html><body><h1>  \n  Spaced H1  \t  </h1></body></html>";
    assert_eq!(extract_title_from_html(html), "Spaced H1 (H1 tag)");
}

// -- validate_url --

#[test]
fn test_validate_url_valid_https() {
    let result = validate_url("https://example.com");
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.scheme(), "https");
    assert_eq!(parsed.host_str(), Some("example.com"));
}

#[test]
fn test_validate_url_valid_http() {
    let result = validate_url("http://example.com/path/to/page");
    assert!(result.is_ok());
    let parsed = result.unwrap();
    assert_eq!(parsed.scheme(), "http");
    assert_eq!(parsed.path(), "/path/to/page");
}

#[test]
fn test_validate_url_invalid_parse() {
    let result = validate_url("not a valid url");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid URL"));
}

#[test]
fn test_validate_url_invalid_scheme_ftp() {
    let result = validate_url("ftp://files.example.com/file.txt");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("scheme"));
}

#[test]
fn test_validate_url_invalid_scheme_file() {
    let result = validate_url("file:///etc/passwd");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("scheme"));
}

#[test]
fn test_validate_url_missing_host() {
    // "http://" fails at the URL parse stage (empty host is a parse error).
    let result = validate_url("http://");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid URL"));
}

#[test]
fn test_validate_url_invalid_scheme_data() {
    // Data URIs are valid URLs but not fetchable web pages.
    let result = validate_url("data:text/html,<h1>Hello</h1>");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("scheme"));
}

#[test]
fn test_validate_url_with_port() {
    let result = validate_url("http://localhost:8080/api");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().port(), Some(8080));
}

#[test]
fn test_validate_url_with_query() {
    let result = validate_url("https://example.com/search?q=rust&page=1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().query(), Some("q=rust&page=1"));
}
