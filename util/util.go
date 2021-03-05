package util

import (
	"golang.org/x/text/encoding/charmap"
	"io"
	"log"
	"os"
	"regexp"
)

func CheckErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

func Copy(src, dst string) error {
	in, err := os.Open(src)
	CheckErr(err)
	defer in.Close()

	out, err := os.Create(dst)
	CheckErr(err)
	defer out.Close()

	_, err = io.Copy(out, in)
	CheckErr(err)
	return out.Close()
}

var leftBracketRegex = regexp.MustCompile(`\[`)
var rightBracketRegex = regexp.MustCompile(`\]`)

func DecodeWindows1250(input []byte) ([]byte, error) {
	utf8, err := charmap.Windows1250.NewDecoder().Bytes(input)

	leftReplaced := leftBracketRegex.ReplaceAllString(string(utf8), "(")
	rightReplaced := rightBracketRegex.ReplaceAllString(leftReplaced, ")")

	return []byte(rightReplaced), err
}
