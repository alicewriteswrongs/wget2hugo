package converter

import "testing"

var input = `<p>first [literal] brackets</p>
<p>then &#91;one&#93; way to escape</p>
<p>then &lbrack;another&rbrack; one</p>`

var output = `first [literal] brackets

then [one] way to escape

then [another] one`

func TestConvertBrackets(t *testing.T) {
	mdchan := make(chan []byte)
	go Convert([]byte(input), mdchan)
	markdown := <-mdchan

	if string(markdown) != output {
		t.Errorf(`Failed ! got "%v" want []`, string(markdown))
	} else {
		t.Logf("Success !")
	}
}
