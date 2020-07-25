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

It's written in Go, using [this HTML -> Markdown
library](https://github.com/JohannesKaufmann/html-to-markdown). I wrote a
previous version in node.js using
[turndown](https://github.com/domchristie/turndown), but ran into issues with
memory leaks and performance. 

### Running it

You need to copy your backup into the project working directory.
Right now it expects it at `./private/input`

To build the project do:

```sh
go build convert.go
./convert
```

You can also just run it by doing

```sh
go run convert.go
```

This will convert the HTML in the site backup into Markdown, and it will copy
it (along with all PDF, .doc, image files, etc) into `private/output/` in a
format that will work with Hugo. You should then be able to just copy that
whole directory into your Hugo site's `content` directory.


### TODO

- rewrite as a binary that takes `-s` source and `-d` destination flags
- convert HTML text encoding if not UTF-8
- fix anchor tags to anchors within the page
