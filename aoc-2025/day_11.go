package main

import (
	"fmt"
	"strings"
	"time"
)

type DeviceOutput []string
type Device string
type D11Input map[Device]DeviceOutput

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

	devices := make(map[Device]DeviceOutput)

	for line := range strings.SplitSeq(input, "\n") {
		d := strings.Split(line, ": ")
		device := d[0]
		outputs := strings.Split(d[1], " ")

		devices[Device(device)] = outputs
	}

	return devices
}

func d11ex1(input D11Input) int {

	cache := make(map[string]int)

	return dfs(input, "you", cache)
}

func dfs(input D11Input, part string, cache map[string]int) int {

	if part == "out" {
		return 1
	}

	s, h := cache[part]
	if h {
		return s
	}

	sum := 0

	outputs, _ := input[Device(part)]

	for _, f := range outputs {
		sum += dfs(input, f, cache)
	}

	cache[part] = sum

	return sum
}

func dfs2(input D11Input, part string, cache map[D11Entry]int, dac, fft bool) int {

	if part == "out" && dac && fft {
		return 1
	}

	if part == "dac" {
		dac = true
	}
	if part == "fft" {
		fft = true
	}

	key := D11Entry{
		part: part,
		ftt:  fft,
		dac:  dac,
	}

	s, h := cache[key]
	if h {
		return s
	}

	sum := 0

	outputs, _ := input[Device(part)]

	for _, f := range outputs {
		sum += dfs2(input, f, cache, dac, fft)
	}

	cache[key] = sum

	return sum
}

func d11ex2(input D11Input) int {
	cache := make(map[D11Entry]int)

	return dfs2(input, "svr", cache, false, false)
}

type D11Entry struct {
	part string
	ftt  bool
	dac  bool
}
