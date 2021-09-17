use clap::{AppSettings, Clap};
use std::fs;
use std::path::Path;
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

    let source_dir_path = Path::new(&opts.source);
    let destination_dir_path = Path::new(&opts.destination);

    for entry in WalkDir::new(&opts.source) {
        // I think it's ok to unwrap this here since we
        // don't really mind if this panics (if it does, we want
        // to not do anything anyhow)
        let direntry = entry.unwrap();
        let source_path = direntry.path();

        let destination_path = source_path
            .strip_prefix(source_dir_path)
            .map(|relative_path| destination_dir_path.join(relative_path));

        match destination_path {
            Ok(mut destination_path) => {
                println!("source path:      {}", source_path.display());
                println!("destination path: {}", destination_path.display());

                let extension = destination_path
                    .extension()
                    .map(|os| os.to_str().unwrap());

                let operation = match extension {
                    // we've got a directory
                    None => fs::create_dir(destination_path),
                    // we've got a file (or something with an extension!)
                    Some("htm") | Some("html") => fs::read(source_path)
                        .map(conversion::bytes_to_utf8)
                        .map(conversion::html_to_markdown)
                        .map(|markdown| {
                            destination_path.set_extension("md");
                            fs::write(destination_path, markdown).unwrap()
                        }),
                    Some(_ext) => {
                        // this is some other file (maybe a pdf or an image)
                        // so we just want to copy it
                        fs::copy(source_path, destination_path).map(|_number| ())
                    }
                };
            }
            Err(error) => println!("some error: {}", error),
        }
    }
}
