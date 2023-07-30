package p1

import (
	"errors"
	"fmt"
	"os"
	"testing"
)

func TestP1(t *testing.T) {
	file, err := os.ReadFile("p1.input")

	if err != nil {
		fmt.Println("err on read file: ", err)
	}

	var want int = 69281
	result, err := p1(string(file))

	if result != want {
		t.Fatalf("Expected: %d, Actual: %d", want, result)
	}
}

func TestP1Empty(t *testing.T) {
	result, err := p1("")
	if err == nil || result != 0 {
		t.Fatal(errors.New("empty input"))
	}
}
