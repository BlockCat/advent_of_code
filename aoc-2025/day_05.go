package main

import (
	"fmt"
)

func main() {

	var input, err = readFile(5)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var ex1 = 0
	var ex2 = 0

	fmt.Printf("Ex 1: %d\n", ex1)
	fmt.Printf("Ex 2: %d\n", ex2)
	// fmt.Printf("Ex 2: %d joltage\n", sum2)

}
