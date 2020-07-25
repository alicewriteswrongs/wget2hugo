package main

import (
	"github.com/aliceriot/wget2hugo/converter"
	"github.com/aliceriot/wget2hugo/util"

	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var replacer = strings.NewReplacer(
	"private/input",
	"private/output",
	" ",
	"_",
)

var indexMDRegex = regexp.MustCompile(`index.html$|index.htm`)

var mdRegex = regexp.MustCompile(`.html$|.htm$`)

func Walker(path string, info os.FileInfo, err error) error {
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
		markdown, err := converter.Convert(contents)
		util.CheckErr(err)

		var markdownPath string
		if indexMDRegex.MatchString(newpath) {
			markdownPath = indexMDRegex.ReplaceAllString(newpath, "_index.md")
		} else {
			markdownPath = mdRegex.ReplaceAllString(newpath, ".md")
		}

		fmt.Println("writing markdown: " + newpath)
		err = ioutil.WriteFile(markdownPath, markdown, info.Mode().Perm())
		util.CheckErr(err)
		return nil
	} else {
		// else it's a PDF, word doc, image, etc and we just want to copy it
		fmt.Println("copying file: " + newpath)
		util.Copy(path, newpath)
		return nil
	}
}

func main() {
	filepath.Walk("private/input", Walker)
}
