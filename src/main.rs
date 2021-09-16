use clap::{AppSettings, Clap};
use std::fs;
use walkdir::WalkDir;

mod conversion;

#[derive(Clap)]
#[clap(name = "wget2hugo", version = "0.0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(short, long)]
    source: String,

    #[clap(short, long)]
    destination: String,
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

                let markdown = fs::read(path)
                    .map(conversion::bytes_to_utf8)
                    .map(conversion::html_to_markdown);

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
