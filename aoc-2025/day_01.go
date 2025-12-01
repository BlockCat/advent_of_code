package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("hello world")

	var input, err = readFile(1)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var knob = newKnob()

	var counter = 0
	var counter2 = 0

	for val := range strings.SplitSeq(input, "\n") {
		var d = val[0]
		var v, e = strconv.Atoi(val[1:])

		if e != nil {
			panic("Could not parse" + e.Error())
		}

		switch d {
		case 'L':
			counter2 += knob.left(v)
		case 'R':
			counter2 += knob.right(v)
		default:
			panic("Unreachable")
		}

		if knob.isNull() {
			counter++
		}
	}

	fmt.Printf("Ex1: Pointed to 0 exactly %d times\n", counter)
	fmt.Printf("Ex2: Clicked to 0 exactly %d times\n", counter2)
}

type knob struct {
	value int
	size  int
}

func newKnob() *knob {
	return &knob{value: 50, size: 100}
}

func (knob *knob) isNull() bool {
	return knob.value == 0
}

func (knob *knob) right(val int) int {
	var clicks = val / knob.size
	knob.value += val % knob.size

	if knob.value >= knob.size {
		knob.value -= knob.size
		clicks++
	}

	if knob.value < 0 || knob.value >= knob.size {
		panic("Invalid range")
	}

	return clicks
}

func (knob *knob) left(val int) int {

	var clicks = val / knob.size

	if knob.value == 0 {
		clicks--
	}
	knob.value -= val % knob.size

	if knob.value < 0 {
		knob.value += knob.size
		clicks++
	}

	if knob.value == 0 {
		clicks++
	}

	if knob.value < 0 || knob.value >= knob.size {
		panic("Invalid range")
	}

	return clicks
}
