package main

import (
	"cmp"
	"fmt"
	"reflect"
	"slices"
	"strconv"
	"strings"
	"time"
)

type D10Input []MachineStatement

type MachineStatement struct {
	scheme  []bool
	buttons [][]int
	joltage []int
	cacher  Cacher
}

const MAX_VALUE int = 1000000000000000000

func main() {

	var input, err = readFile(10)

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	start := time.Now()

	preprocessed := d10preprocess(input)

	preprocessTime := time.Now().Sub(start)
	start = time.Now()

	// ex1 := d10ex1(preprocessed)

	ex1elapsed := time.Now().Sub(start)

	// fmt.Printf("Ex 1: %d\n", ex1)

	start = time.Now()

	ex2 := d10ex2p(preprocessed)

	ex2elapsed := time.Now().Sub(start)

	fmt.Printf("Ex 2: %d\n", ex2)

	fmt.Printf("Preprocess: %s\n", preprocessTime.String())
	fmt.Printf("Ex1 time: %s\n", ex1elapsed.String())
	fmt.Printf("Ex2 time: %s\n", ex2elapsed.String())

}

func d10preprocess(input string) D10Input {
	lines := strings.Split(input, "\n")

	machines := make([]MachineStatement, len(lines))

	for i, line := range lines {
		parts := strings.Fields(line)

		indicatorLights := parts[0]

		wiringSchematics := parts[1 : len(parts)-1]
		joltageRequirements := parts[len(parts)-1]

		scheme := make([]bool, len(indicatorLights)-2)
		buttons := make([][]int, len(wiringSchematics))

		for li, c := range indicatorLights[1 : len(indicatorLights)-1] {
			switch c {
			case '#':
				scheme[li] = true
			case '.':
				scheme[li] = false
			default:
				panic("unreachable")
			}
		}

		for li, w := range wiringSchematics {
			w = w[1 : len(w)-1]
			prepNumber := strings.Split(w, ",")
			numbers := make([]int, len(prepNumber))
			for pni, ns := range prepNumber {
				r, e := strconv.Atoi(ns)
				if e != nil {
					panic(e)
				}
				numbers[pni] = r
			}
			buttons[li] = numbers
		}

		slices.SortFunc(buttons, func(a, b []int) int {
			return cmp.Compare(len(a), len(b))
		})

		joltageRequirements = joltageRequirements[1 : len(joltageRequirements)-1]
		prepNumber := strings.Split(joltageRequirements, ",")

		joltage := make([]int, len(prepNumber))
		for jni, ns := range prepNumber {
			r, e := strconv.Atoi(ns)
			if e != nil {
				panic(e)
			}
			joltage[jni] = r
		}

		machines[i] = MachineStatement{
			scheme:  scheme,
			buttons: buttons,
			joltage: joltage,
			cacher: Cacher{
				cache: make(map[uint64]CacheLane),
			},
		}
	}

	return machines
}

func d10ex1(input D10Input) int {

	sum := 0

	// ch := make(chan int)

	for i, ms := range input {
		// go func() {
		r := turnMachineOnLeastPresses(ms)
		fmt.Println("machine ", i, "takes ", r, "presses, total: ", sum)
		// ch <- r
		// }()
	}

	// for range input {
	// 	sum += <-ch
	// }

	return sum
}

func d10ex2(input D10Input) int {
	sum := 0

	for i, ms := range input {

		r := provideJoltageToMachines2(ms)
		sum += r

		fmt.Println("machine ", i, "takes ", r, "presses, total: ", sum)

	}

	return sum
}
func d10ex2p(input D10Input) int {
	sum := 0

	ch := make(chan int)
	machinesLeft := len(input)

	for i, ms := range input {
		go func() {
			r := provideJoltageToMachines2(ms)
			ch <- r

			fmt.Println("machine ", i, "takes ", r, "presses, total: ", sum)
		}()
	}

	for range input {
		sum += <-ch
		machinesLeft--
		fmt.Println("There are ", machinesLeft, " machines left")
	}

	return sum
}

func turnMachineOnLeastPresses(ms MachineStatement) int {

	type QueueRecord struct {
		state []bool
		dist  int
	}

	queue := make([]QueueRecord, 1)
	queue[0] = QueueRecord{
		state: make([]bool, len(ms.scheme)),
		dist:  0,
	}

	for len(queue) > 0 {
		next := queue[0]
		queue = queue[1:]

		if reflect.DeepEqual(next.state, ms.scheme) {
			return next.dist
		}

		for _, button := range ms.buttons {
			candidate := pressButton(next.state, button)
			queue = append(queue, QueueRecord{
				state: candidate,
				dist:  next.dist + 1,
			})
		}
	}
	panic("unreachable")
}

func provideJoltageToMachines2(ms MachineStatement) int {
	return ProvideJoltageToMachineSub(&ms, 0, 0, make([]int, len(ms.joltage)))
}

// It's like a vector:
// ax1 + bx2 + .... = r

func ProvideJoltageToMachineSub(ms *MachineStatement, part, depth int, state []int) int {

	if part == len(ms.joltage) {
		return depth
	}

	relevantButtons := CalculateRelevantButtons(ms, part)
	requiredJoltageAtLevel := ms.joltage[part]
	current := state[part]

	requiredPresses := requiredJoltageAtLevel - current

	// fmt.Println(relevantButtons, part, depth, requiredPresses, state)

	if requiredPresses == 0 {
		return ProvideJoltageToMachineSub(ms, part+1, depth, state)
	} else if requiredPresses < 0 {
		panic("")
	}

	// try all combinations?
	cacheItem := ms.cacher.HasCacheEntry(part, state)

	if cacheItem == nil {
		result := ButtonsTry(ms, relevantButtons, requiredPresses, part, depth, state)
		cacheItem := ms.cacher.AddCacheEntry(*NewCacheEntry(part, result, state))

		if cacheItem != nil {
			panic("expected cache to not have it nil")
		}
		return result
	} else {
		// fmt.Println("cache hit at part, ", part)
		return cacheItem.depth
	}

}

func ButtonsTry(ms *MachineStatement, buttons [][]int, remaining, part, depth int, state []int) int {
	if len(buttons) == 0 {
		return MAX_VALUE
	}
	if len(buttons) == 1 {
		nextState := provideJoltage(state, buttons[0], remaining)
		if CheckJoltageValid(ms, nextState) {
			return ProvideJoltageToMachineSub(ms, part+1, depth+remaining, nextState)
		} else {
			return MAX_VALUE
		}

	}
	minValue := MAX_VALUE
	for i := range remaining + 1 {
		nextState := provideJoltage(state, buttons[0], i)
		if CheckJoltageValid(ms, nextState) {
			r := ButtonsTry(ms, buttons[1:], remaining-i, part, depth+i, nextState)
			minValue = min(minValue, r)
		}
	}

	return minValue
}

func CheckJoltageValid(ms *MachineStatement, state []int) bool {
	for i, max := range ms.joltage {
		if state[i] > max {
			return false
		}
	}
	return true
}

func CalculateRelevantButtons(ms *MachineStatement, part int) [][]int {
	relevantButtons := make([][]int, 0)
	for _, button := range ms.buttons {
		if !IsButtonRelevent(part, button) {
			continue
		}

		relevantButtons = append(relevantButtons, button)
	}
	return relevantButtons
}

func IsButtonRelevent(part int, button []int) bool {

	contains := false
	for _, v := range button {
		if v == part {
			contains = true
		}
		if v < part {
			return false
		}
	}
	return contains
}

func pressButton(state []bool, button []int) []bool {
	nextState := make([]bool, len(state))
	copy(nextState, state)

	for _, i := range button {
		nextState[i] = !nextState[i]
	}

	return nextState
}

func provideJoltage(state []int, button []int, sum int) []int {
	nextState := make([]int, len(state))
	copy(nextState, state)

	for _, i := range button {
		nextState[i] += sum
	}

	return nextState
}

type Cacher struct {
	cache map[uint64]CacheLane
}

func (c *Cacher) AddCacheEntry(entry CacheEntry) *CacheEntry {

	result := c.HasCacheEntry(entry.part, entry.state)

	if result == nil {
		hash := entry.Hash()
		l, h := c.cache[hash]
		if h {
			l = append(l, entry)
			c.cache[hash] = l
		} else {
			c.cache[hash] = []CacheEntry{entry}
		}
		return nil
	}

	old := NewCacheEntry(result.part, result.depth, result.state)

	result.depth = min(result.depth, entry.depth)

	return old
}

func (c *Cacher) HasCacheEntry(part int, state []int) *CacheEntry {
	hash := (&CacheEntry{part: part, state: state}).Hash()

	l, h := c.cache[hash]

	if h {
		index := slices.IndexFunc(l, func(e CacheEntry) bool {
			return e.part == part && reflect.DeepEqual(e.state, state)
		})

		if index == -1 {
			return nil
		}

		return &l[index]
	}
	return nil

}

type CacheLane []CacheEntry
type CacheEntry struct {
	state []int
	part  int
	depth int
}

func NewCacheEntry(part, depth int, state []int) *CacheEntry {
	entry := CacheEntry{
		state: make([]int, len(state)),
		part:  part,
		depth: depth,
	}
	copy(entry.state, state)

	return &entry
}

func (ce *CacheEntry) Hash() uint64 {
	sum := uint64(ce.part)

	for _, j := range ce.state {
		sum *= 100
		sum += uint64(j)
	}

	return sum
}
