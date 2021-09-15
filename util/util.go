package util

import (
	"io"
	"log"
	"os"
	"time"

	"golang.org/x/text/encoding/charmap"
)

func CheckErr(err error) {
	if err != nil {
		log.Fatal(err)
	}
}

// returns true if we should copy src to dest (because
// src is newer than dest)
func CheckShouldCopy(src, dest string) bool {
	// first check modification times and avoid copying
	// if the destination is newer than the source
	srcInfo, err := os.Stat(src)
	CheckErr(err)
	destInfo, err := os.Stat(dest)
	CheckErr(err)

	srcModTime := srcInfo.ModTime()
	destModTime := destInfo.ModTime()

	diff := destModTime.Sub(srcModTime)

	return diff < (time.Duration(0) * time.Second)
}

func Copy(src, dest string) error {
	in, err := os.Open(src)
	CheckErr(err)
	defer in.Close()

	out, err := os.Create(dest)
	CheckErr(err)
	defer out.Close()

	_, err = io.Copy(out, in)

	CheckErr(err)
	return out.Close()
}

func DecodeWindows1250(input []byte) ([]byte, error) {
	utf8, err := charmap.Windows1250.NewDecoder().Bytes(input)
	return utf8, err
}
