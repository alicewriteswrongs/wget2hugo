use chardetng::EncodingDetector;
use clap::{AppSettings, Clap};
use html2md::parse_html;
use std::fs;
use walkdir::WalkDir;

#[derive(Clap)]
#[clap(name = "wget2hugo", version = "0.0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(short, long)]
    source: String,

    #[clap(short, long)]
    destination: String,
}

fn convert_to_utf8(bytes: Vec<u8>) -> String {
    let mut detector = EncodingDetector::new();
    let slice = bytes.as_slice();
    detector.feed(slice, true);
    let encoding = detector.guess(None, true);
    let (converted, _used, _malformed) = encoding.decode(slice);
    String::from(converted)
}

// Take an HTML string and convert it to Markdown using
// html2md::parse_html
fn convert_to_markdown(html: String) -> String {
    parse_html(&html)
}

fn main() {
    let opts: Options = Options::parse();

    for entry in WalkDir::new(opts.source) {
        let direntry = entry.unwrap();
        let path = direntry.path();

        let extension = path.extension().map(|os| os.to_str().unwrap());

        match extension {
            // we've got a directory
            None => {
                println!("found directory: {}", path.display());
            }
            // we've got a file (or something with an extension!)
            Some("htm") | Some("html") => {
                println!("found html {}", path.display());

                let markdown = fs::read(path).map(convert_to_utf8).map(convert_to_markdown);

                match markdown {
                    Ok(md) => println!("converted! {}", md),
                    Err(err) => println!("some error {}", err),
                }
            }
            Some(ext) => {
                println!("another ext:  {}", ext);
                println!("at path: {}", path.display());
            }
        }
    }
}
