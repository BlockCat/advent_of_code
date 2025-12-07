package main

import (
	"fmt"
	"strings"
	"time"
)

// v1:
// prep: 0.148ms
// ex1:  0.366ms
// ex2:  0.369ms

// v2:
// all: 0.284\ms

func main() {

	var input, err = readFile(7)

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	start := time.Now()

	ex1, ex2 := d7(input)
	ex1elapsed := time.Now().Sub(start)
	fmt.Printf("Ex 1: %d\n", ex1)
	fmt.Printf("Ex 2: %d\n", ex2)

	fmt.Printf("all: %s", ex1elapsed.String())

}

func d7(input string) (int, int) {
	lines := strings.SplitSeq(input, "\n")

	startPos := strings.Index(input, "S")

	// sweepEx1 := make(map[int]int)
	sweepEx2 := make(map[int]int)

	// sweepEx1[startPos] = 1
	sweepEx2[startPos] = 1

	ex1 := 0

	for line := range lines {
		for k, ex2v := range sweepEx2 {

			cell := line[k]

			switch cell {
			case '.':
				continue
			case 'S':
				continue
			case '^':
				ex1++

				sweepEx2[k-1] += ex2v
				sweepEx2[k+1] += ex2v

				delete(sweepEx2, k)
			default:
				panic("unexpected: " + string(cell))
			}
		}

	}

	ex2 := 0
	for _, v := range sweepEx2 {
		ex2 += v
	}

	return ex1, ex2
}
