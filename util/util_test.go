package util

import "testing"

func TestDecodeWindows1250(t *testing.T) {
	input := []byte{91, 93}
	output, _ := DecodeWindows1250(input)

	if string(output) != "[]" {
		t.Errorf("Failed ! got %v want []", output)
	} else {
		t.Logf("Success !")
	}
}
