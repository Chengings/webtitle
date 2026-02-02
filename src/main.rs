//! Fetch the `<title>` of a website from a given URL, falling back to the
//! first `<h1>` tag. Outputs the title as plain text and as a Markdown link.

use anyhow::{Result, bail};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector};
use std::env;
use std::process::exit;
use std::time::Duration;
use url::Url;

/// Extracts the page title from an HTML document.
///
/// Precedence: `<title>` tag, then first `<h1>` (suffixed with " (H1 tag)"),
/// then "No title found".
fn extract_title_from_html(html: &str) -> String {
    let document = Html::parse_document(html);

    // .unwrap() is safe: these selectors are compile-time constants.
    let title_selector = Selector::parse("title").unwrap();
    let h1_selector = Selector::parse("h1").unwrap();

    if let Some(el) = document.select(&title_selector).next() {
        let text: String = el.text().collect();
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }

    if let Some(el) = document.select(&h1_selector).next() {
        let text: String = el.text().collect();
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            return format!("{} (H1 tag)", trimmed);
        }
    }

    "No title found".to_string()
}

/// Validates that a URL string is parseable and uses an http(s) scheme with a host.
fn validate_url(url: &str) -> Result<Url> {
    let parsed = match Url::parse(url) {
        Ok(u) => u,
        Err(_) => bail!("Invalid URL"),
    };

    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        bail!("Invalid URL scheme (must be http or https)");
    }

    if parsed.host().is_none() {
        bail!("Invalid URL (missing host)");
    }

    Ok(parsed)
}

/// Fetches the page at `url` and extracts its title.
///
/// Uses a blocking HTTP client with browser-like headers and a 10s timeout.
fn get_website_title(url: &str) -> Result<String> {
    let mut headers = HeaderMap::new();

    // Mimic a real browser; many sites return 403 for bare programmatic User-Agents.
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36",
        ),
    );
    // Request English content for consistent title language.
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));

    // Blocking client is appropriate for a single-request CLI tool.
    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.get(url).send()?.error_for_status()?;
    let body = response.text()?;

    Ok(extract_title_from_html(&body))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: webtitle <url>");
        exit(1);
    }

    let url = &args[1];

    let _parsed = match validate_url(url) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    };

    let title = match get_website_title(url) {
        Ok(t) => t,
        Err(e) => bail!("Error: {}", e),
    };

    println!("{}", title);
    println!("[{}]({})", title, url);

    Ok(())
}

#[cfg(test)]
mod tests;
