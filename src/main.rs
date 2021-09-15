use clap::{AppSettings, Clap};
use html2md::parse_html;
use std::ffi::OsStr;
use walkdir::WalkDir;

#[derive(Clap)]
#[clap(name = "wget2hugo", version = "0.0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Options {
    #[clap(short, long)]
    source: String,

    #[clap(short, long)]
    destination: String,

    #[clap(short, long)]
    convert_from_1252: Option<bool>,
}

fn main() {
    let opts: Options = Options::parse();

    println!("Hello, world!");
    println!("source: {}", opts.source);
    println!("dest: {}", opts.destination);

    // let foo = WalkDir::new(opts.source);

    for entry in WalkDir::new(opts.source) {
        let direntry = entry.unwrap();
        let path = direntry.path();

        let extension = path.extension().map(|os| os.to_str());

        match extension {
            // we've got a directory
            None => {
                println!("found directory: {}", path.display());
            },
            // we've got a file (or something with an extension!)
            Some(string_ext) => {
                match string_ext {
                    Some("htm") | Some("html") => {
                        println!("found html {}", path.display());
                    }
                    Some(ext) => {
                        println!("another ext:  {}", ext);
                        println!("at path: {}", path.display());
                    },
                    None => {
                        println!("something went wrong");
                    }
                }
            }
        }
    }
}
