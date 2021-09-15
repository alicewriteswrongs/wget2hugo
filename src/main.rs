use clap::{AppSettings, Clap};

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
}
