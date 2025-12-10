package main

import (
	"fmt"
	"strings"
	"time"
)

type D11Input []int

func main() {

	var input, err = readFile(11)

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	start := time.Now()

	preprocessed := d11preprocess(input)

	preprocessTime := time.Now().Sub(start)
	start = time.Now()

	ex1 := d11ex1(preprocessed)

	ex1elapsed := time.Now().Sub(start)

	fmt.Printf("Ex 1: %d\n", ex1)

	start = time.Now()

	ex2 := d11ex2(preprocessed)

	ex2elapsed := time.Now().Sub(start)

	fmt.Printf("Ex 2: %d\n", ex2)

	fmt.Printf("Preprocess: %s\n", preprocessTime.String())
	fmt.Printf("Ex1 time: %s\n", ex1elapsed.String())
	fmt.Printf("Ex2 time: %s\n", ex2elapsed.String())

}

func d11preprocess(input string) D11Input {
	lines := strings.Split(input, "\n")

	for i, line := range lines {
	}
}

func d11ex1(input D11Input) int {
	return 0
}

func d11ex2(input D11Input) int {
	return 0
}
