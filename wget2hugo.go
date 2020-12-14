package main

import (
	"github.com/aliceriot/wget2hugo/converter"
	"github.com/aliceriot/wget2hugo/util"

	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var indexMDRegex = regexp.MustCompile(`index.html$|index.htm`)

var mdRegex = regexp.MustCompile(`.html$|.htm$`)

func Walker(path string, info os.FileInfo, err error) error {
	replacer := strings.NewReplacer(
		source,
		destination,
		" ",
		"_",
	)

	newpath := strings.ToLower(replacer.Replace(path))

	// if directory we want to just create the new dir
	if info.IsDir() {
		err := os.MkdirAll(newpath, info.Mode().Perm())
		util.CheckErr(err)
		fmt.Println("created dir: " + newpath)
		return nil
	}

	// if it's an html file we want to convert to markdown
	if filepath.Ext(path) == ".htm" || filepath.Ext(path) == ".html" {
		contents, err := ioutil.ReadFile(path)
		util.CheckErr(err)

		mdchan := make(chan []byte)
		go converter.Convert(contents, mdchan)
		markdown := <-mdchan

		var markdownPath string
		if indexMDRegex.MatchString(newpath) {
			markdownPath = indexMDRegex.ReplaceAllString(newpath, "_index.md")
		} else {
			markdownPath = mdRegex.ReplaceAllString(newpath, ".md")
		}

		fmt.Println("writing markdown: " + markdownPath)
		err = ioutil.WriteFile(markdownPath, markdown, info.Mode().Perm())
		util.CheckErr(err)
		return nil
	} else {
		// else it's a PDF, word doc, image, etc and we just want to copy it
		fmt.Println("copying file: " + newpath)
		go util.Copy(path, newpath)
		return nil
	}
}

var source string
var destination string

func main() {
	flag.StringVar(&source, "source", "", "location of wget backup source")
	flag.StringVar(&destination, "destination", "", "output directory")

	flag.Parse()

	if flag.NFlag() == 0 {
		fmt.Println("wget2hugo: convert a wget backup of a website to hugo-compatible Markdown")
		flag.PrintDefaults()
	} else {
		fmt.Println(source)
		fmt.Println(destination)
		filepath.Walk(source, Walker)
	}
}
