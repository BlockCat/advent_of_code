package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {

	var input, err = readFile(6)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	lines := strings.Split(input, "\n")

	fmt.Printf("Ex 1: %d\n", ex1(lines[0:len(lines)-1], lines[len(lines)-1]))
	fmt.Printf("Ex 2: %d\n", ex2(lines[0:len(lines)-1], lines[len(lines)-1]))
}

func ex1(lines []string, modeLine string) int {
	ex1 := 0

	mode := strings.Fields(modeLine)

	grid := make([][]string, len(lines))

	for l := range len(lines) {
		grid[l] = strings.Fields(lines[l])
	}

	for i := range len(mode) {

		switch mode[i] {
		case "+":
			sum := 0
			for _, line := range grid {
				r, _ := strconv.Atoi(strings.TrimSpace(line[i]))
				sum += r
			}
			ex1 += sum
		case "*":
			sum := 1
			for _, line := range grid {
				r, _ := strconv.Atoi(strings.TrimSpace(line[i]))
				sum *= r
			}
			ex1 += sum
		default:
			panic("unexpeced: " + mode[i])
		}
	}

	return ex1
}

func ex2(lines []string, mode string) int {
	maxLen := max(len(lines[0]))
	index := 0

	ex2 := 0
	sum := 0
	mul := false

	for index < maxLen {
		mode := mode[index]
		if mode == '*' {
			ex2 += sum
			mul = true
			sum = 1
		} else if mode == '+' {
			ex2 += sum
			mul = false
			sum = 0
		} else if mode == ' ' {
			mul = mul
		} else {
			panic("huh")
		}

		chars := make([]byte, len(lines))
		for i, r := range lines {
			chars[i] = r[index]
		}

		c, e := strconv.Atoi(strings.TrimSpace(string(chars)))

		if e != nil {
			index++
			continue
		}

		// println(c)

		if mul {
			sum *= c
		} else {
			sum += c
		}
		index++
	}

	ex2 += sum

	return ex2
}
