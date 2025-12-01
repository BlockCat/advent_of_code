package main

import (
	"fmt"
	"os"
)

func readFile(day int) (string, error) {
	var path = fmt.Sprintf("inputs/day_%02d.txt", day)

	data, err := os.ReadFile(path)

	if err != nil {
		return "", err
	}

	return string(data), nil
}
