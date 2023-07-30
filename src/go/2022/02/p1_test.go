package p1

import (
	"fmt"
	"os"
	"testing"
)

func TestP1(t *testing.T) {
	file, err := os.ReadFile("p1.input")

	if err != nil {
		fmt.Println("ReadFile err: ", err)
	}

	expect := 12772
	actual, err := p1(string(file))

	if err != nil {
		t.Fatal("p1 err: ", err)
	}

	if expect != actual {
		t.Fatalf("Expected: %d, Actual: %d", expect, actual)
	}
}

func TestP1Empty(t *testing.T) {
	result, err := p1("")

	if result != 0 || err == nil {
		t.Fatal("d%", result)
	}

}
