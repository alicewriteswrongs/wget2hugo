package converter

import (
	"example.com/test/util"
	"fmt"
	"github.com/JohannesKaufmann/html-to-markdown"
	"github.com/PuerkitoBio/goquery"
	"regexp"
	"strings"
)

var index = regexp.MustCompile(`index.html$|index.htm$|.html$|.htm$`)

func fixLinkHref(href string) string {
	return strings.ToLower(index.ReplaceAllString(href, ""))
}

func getConverter() *md.Converter {
	converter := md.NewConverter("", true, nil)
	converter.AddRules(
		md.Rule{
			Filter: []string{"a"},
			AdvancedReplacement: func(content string, selec *goquery.Selection, opt *md.Options) (md.AdvancedResult, bool) {
				href, ok := selec.Attr("href")
				if !ok {
					return md.AdvancedResult{}, true
				}

				var title string
				if t, ok := selec.Attr("title"); ok {
					title = fmt.Sprintf(` "%s"`, t)
				}

				if opt.LinkStyle == "inlined" {
					return md.AdvancedResult{
						Markdown: fmt.Sprintf("[%s](%s%s)", content, fixLinkHref(href), title),
					}, false
				}

				var replacement string
				var reference string

				switch opt.LinkReferenceStyle {
				case "collapsed":

					replacement = "[" + content + "][]"
					reference = "[" + content + "]: " + href + title
				case "shortcut":
					replacement = "[" + content + "]"
					reference = "[" + content + "]: " + href + title

				default:
					id := selec.AttrOr("data-index", "")
					replacement = "[" + content + "][" + id + "]"
					reference = "[" + id + "]: " + href + title
				}
				return md.AdvancedResult{Markdown: replacement, Footer: reference}, false
			},
		},
	)
	return converter
}

func Convert(data []byte) ([]byte, error) {
	converter := getConverter()

	markdown, err := converter.ConvertBytes(data)
	util.CheckErr(err)
	return markdown, err
}
