# wget2hugo

This is a program that converts a `wget` backup of a site into Markdown, which
can then be used as content in [Hugo](https://gohugo.io/) or a similar static
site generator.

You can create a full backup of a website using

```
wget \
    --mirror \
    --convert-links \
    $URL
```

which is great! but if you don't want to merely store that backup or host an
exact mirror you'll want to convert that backup to a more manageable format.
This program will convert html into Markdown files, and will copy all static
files over as well (PDFs, .doc files, images, etc). The goal is to have output
which can be immediately popped into a Hugo site's `content` directory, built,
and deployed.

It's written in Rust, using [this HTML -> Markdown
crate](https://crates.io/crates/html2md). I wrote a
previous version in node.js using
[turndown](https://github.com/domchristie/turndown), but ran into issues with
memory leaks and performance.

### Running it

Just do

```sh
cargo run --help
```

and it should print usage information.
