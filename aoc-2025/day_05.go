package main

import (
	"cmp"
	"fmt"
	"slices"
	"strconv"
	"strings"
	"time"
)

func main() {

	var input, err = readFile(5)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	loadTime := time.Now()

	intervals, products := load(input)

	fmt.Printf("preprocess: %s\n", time.Now().Sub(loadTime).String())

	var ex1 = 0
	var ex2 = uint64(0)

	var lowerLimit = 1000000000000000000
	var upperLimit = 0

	ex1Time := time.Now()

	for _, interval := range intervals {
		if interval.start < lowerLimit {
			lowerLimit = interval.start
		}
		if interval.end > upperLimit {
			upperLimit = interval.end
		}
	}

	for _, p := range products {
		pn, e := strconv.Atoi(p)
		if e != nil {
			panic("nop2")
		}

		if pn < lowerLimit || pn > upperLimit {
			continue
		}

		for _, interval := range intervals {
			if interval.Has(pn) {
				ex1++
				break
			}
		}
	}

	fmt.Printf("ex1: %s\n", time.Now().Sub(ex1Time).String())

	ex2Time := time.Now()

	started := 0
	stopped := 0

	for _, r := range intervals {
		started = max(r.start, started)
		stopped = r.end

		if stopped >= started {
			ex2 += uint64(stopped - started + 1)
		}

		started = stopped + 1

	}

	fmt.Printf("ex2: %s\n", time.Now().Sub(ex2Time).String())

	fmt.Printf("Ex 1: %d\n", ex1)
	fmt.Printf("Ex 2: %d\n", ex2)
	// fmt.Printf("Ex 2: %d joltage\n", sum2)

}

func load(input string) ([]Interval, []string) {

	parts := strings.Split(strings.TrimSpace(input), "\n\n")

	lines := strings.Split(strings.TrimSpace(parts[0]), "\n")
	products := strings.Split(strings.TrimSpace(parts[1]), "\n")

	intervals := make([]Interval, len(lines))

	for i, r := range lines {
		bounds := strings.Split(r, "-")
		l, el := strconv.Atoi(bounds[0])
		r, er := strconv.Atoi(bounds[1])
		if el != nil || er != nil {
			panic("nop")
		}

		intervals[i] = MakeInterval(l, r)
	}

	icmp := func(a, b Interval) int {
		s := cmp.Compare(a.start, b.start)
		if s == 0 {
			return cmp.Compare(a.end, b.end)
		}
		return s
	}

	slices.SortFunc(intervals, icmp)

	return intervals, products
}

type Interval struct {
	start int
	end   int
}

func MakeInterval(start int, end int) Interval {
	return Interval{start: start, end: end}
}

func (i *Interval) Has(n int) bool {
	return n >= i.start && n <= i.end
}
