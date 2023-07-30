package p1

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

const (
	ROCK int = iota + 1
	PAPER
	SCISSOR
)

func p1(input string) (*int, error) {
	if input == "" {
		return nil, errors.New("Input is empty")
	}
	contents := strings.Split(strings.ReplaceAll(strings.TrimSpace(input), " ", ""), "\n")
	winningCombination := map[int]int{ROCK: SCISSOR, PAPER: ROCK, SCISSOR: PAPER}
	Outcome := map[string]int{"WIN": 6, "DRAW": 3, "LOSE": 0}
	PlayerHands := map[string]int{"X": ROCK, "Y": PAPER, "Z": SCISSOR}
	OpponentHands := map[string]int{"A": ROCK, "B": PAPER, "C": SCISSOR}

	points := 0
	for _, item := range contents {
        var op, pl string = string(item[0]), string(item[1])
		if OpponentHands[op] == PlayerHands[pl] {
			points += Outcome["DRAW"]
		}
		if winningCombination[PlayerHands[pl]] == OpponentHands[op] {
			points += Outcome["WIN"]
		}
		points += PlayerHands[pl]
	}
	return &points, nil
}

func main() {
	file, err := os.ReadFile("p1.input")

	if err != nil {
		fmt.Println("err: ", err)
	}

	raw := string(file)

	fmt.Println(p1(raw))

}
