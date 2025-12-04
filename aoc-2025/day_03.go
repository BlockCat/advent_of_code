package main

import (
	"fmt"
	"strings"
)

func main() {

	var input, err = readFile(3)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	sum1 := 0
	sum2 := uint64(0)
	for bank := range strings.Lines(input) {
		bank := strings.Trim(bank, "\n")

		var batteries = []byte(bank)

		sum1 += findHighEx1(batteries)
		sum2 += findHighEx2(batteries, 12)
	}

	fmt.Printf("Ex 1: %d joltage\n", sum1)
	fmt.Printf("Ex 2: %d joltage\n", sum2)

}

func findHighEx1(batteries []byte) int {
	totalLength := len(batteries)
	pos, val := findHighest(batteries[0 : totalLength-1])
	_, val2 := findHighest(batteries[pos+1 : totalLength])

	return val*10 + val2
}

func findHighEx2(batteries []byte, digits int) uint64 {
	totalLength := len(batteries)

	numberBuilder := uint64(0)
	offsetPos := 0
	digits -= 1
	for i := digits; i >= 0; i-- {
		pos, val := findHighest(batteries[offsetPos : totalLength-i])

		// fmt.Println(batteries[offsetPos : totalLength-i])
		offsetPos += pos + 1

		numberBuilder *= 10
		numberBuilder += uint64(val)
	}

	return numberBuilder
}

func findHighest(batteries []byte) (int, int) {
	pos := -1
	val := byte(0)
	counter := 0

	for _, battery := range batteries {
		if battery > val {
			val = battery
			pos = counter
		}
		counter++
	}

	val -= '0'

	return pos, int(val)
}
