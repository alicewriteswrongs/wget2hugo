use chardetng::EncodingDetector;
use html2md::parse_html;

/// Convert a `Vec<u8>` that represents a text file of unknown uncoding
/// into a `String` (i.e. from whatever encoding to utf-8)
///
/// This is particularly handy because `std:fs:read` returns `Vec<u8>`
pub fn bytes_to_utf8(bytes: Vec<u8>) -> String {
    let mut detector = EncodingDetector::new();
    let slice = bytes.as_slice();
    detector.feed(slice, true);
    let encoding = detector.guess(None, true);
    let (converted, _used, _malformed) = encoding.decode(slice);
    String::from(converted)
}

/// Take an HTML string and convert it to Markdown using
/// `html2md::parse_html`
pub fn html_to_markdown(html: String) -> String {
    parse_html(&html)
}
