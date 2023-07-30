package p1

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

func p1(input string) (*int, error) {
	if input == "" {
		return nil, errors.New("empty input")
	}

	contents := strings.Split(input, "\n")
	var count int
	var max int

	for i := 0; i < len(contents); i++ {
		item := contents[i]

		if len(item) == 0 {
			if count > max {
				max = count
			}
			count = 0
			continue
		}

		if number, err := strconv.Atoi(item); err == nil {
			count += number
		} else {
			fmt.Println("err: ", err)
		}
	}
	return &max, nil
}
