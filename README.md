# webtitle

A command-line tool that fetches the `<title>` of a web page, with fallback to
the first `<h1>` tag. Outputs the title as plain text and as a Markdown-formatted
link.

## Usage

```bash
webtitle <url>
```

### Example

```bash
$ webtitle https://example.com
Example Domain
[Example Domain](https://example.com)
```

## Installation

```bash
cargo build --release
# Binary: target/release/webtitle
```

Or via the included justfile:

```bash
just install
```

## Design Decisions

- **Blocking HTTP client** — A single-request CLI tool gains nothing from an
  async runtime. `reqwest::blocking` avoids pulling in `tokio` as a direct
  dependency and keeps the binary lean.
- **Browser User-Agent** — Many sites return 403 or degraded content for
  programmatic user agents. Sending a Chrome-like UA string produces reliable
  results.
- **`<title>` → `<h1>` fallback** — Single-page apps and minimal pages sometimes
  omit `<title>`. Falling back to `<h1>` is a practical heuristic that covers
  most real-world pages.
- **`anyhow` for errors** — Appropriate for a binary crate where callers are
  humans reading stderr, not library consumers matching on error variants.
- **Aggressive release profile** — LTO, symbol stripping, and `panic = "abort"`
  minimize binary size and maximize runtime performance.

## Project Structure

`src/main.rs` contains the core implementation with three clear layers;
`src/tests.rs` holds unit tests in a separate module.

| Function | Responsibility |
|----------|----------------|
| `extract_title_from_html` | Pure HTML parsing — no I/O, fully unit-testable |
| `validate_url` | URL validation (scheme + host checks) |
| `get_website_title` | HTTP fetch with timeout, delegates to parser |
| `main` | Argument parsing and orchestration |

## Testing

```bash
cargo test
```

Unit tests cover HTML title extraction and URL validation across edge cases
(empty titles, whitespace, missing hosts, non-HTTP schemes). Network-dependent
tests are intentionally excluded to keep the suite deterministic and fast.

## Dependencies

| Crate | Purpose |
|-------|---------|
| [`reqwest`](https://crates.io/crates/reqwest) | HTTP client (blocking) |
| [`scraper`](https://crates.io/crates/scraper) | HTML parsing with CSS selectors |
| [`url`](https://crates.io/crates/url) | URL parsing and validation |
| [`anyhow`](https://crates.io/crates/anyhow) | Ergonomic error handling |
