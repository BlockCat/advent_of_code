// package main

// import (
// 	"cmp"
// 	"fmt"
// 	"slices"
// 	"strconv"
// 	"strings"
// 	"time"
// )

// type D9Input [][2]int

// // ex2:
// // 2840002922 too high

// // 290446464 too low
// // 178167411 too low
// // 141227528
// // 139825248 too low
// // // 65107008

// func main() {

// 	var input, err = readFile(9)

// 	if err != nil {
// 		fmt.Println("Error reading file:", err)
// 		return
// 	}

// 	start := time.Now()

// 	preprocessed := d9preprocess(input)

// 	preprocessTime := time.Now().Sub(start)
// 	start = time.Now()

// 	ex1 := d9ex1(preprocessed)

// 	ex1elapsed := time.Now().Sub(start)

// 	fmt.Printf("Ex 1: %d\n", ex1)

// 	start = time.Now()

// 	ex2 := d9ex2(preprocessed)

// 	ex2elapsed := time.Now().Sub(start)

// 	fmt.Printf("Ex 2: %d\n", ex2)

// 	fmt.Printf("Preprocess: %s\n", preprocessTime.String())
// 	fmt.Printf("Ex1 time: %s\n", ex1elapsed.String())
// 	fmt.Printf("Ex2 time: %s\n", ex2elapsed.String())

// }

// func d9preprocess(input string) D9Input {
// 	lines := strings.Split(input, "\n")

// 	coords := make([][2]int, len(lines))

// 	for i, line := range lines {
// 		s := strings.Split(line, ",")
// 		a, _ := strconv.Atoi(s[0])
// 		b, _ := strconv.Atoi(s[1])
// 		coords[i] = [2]int{a, b}
// 	}

// 	// slices.SortFunc(coords, func(a, b [2]int) int {
// 	// 	ycmp := cmp.Compare(a[1], b[1])
// 	// 	if ycmp != 0 {
// 	// 		return ycmp
// 	// 	}
// 	// 	return cmp.Compare(a[0], b[0])
// 	// })

// 	return coords
// }

// func d9ex1(input D9Input) int {
// 	maxRect := 0
// 	for i, aa := range input {
// 		for _, bb := range input[i+1:] {

// 			daa := aa[0] - bb[0]
// 			dbb := aa[1] - bb[1]
// 			if daa < 0 {
// 				daa = -daa
// 			}
// 			if dbb < 0 {
// 				dbb = -dbb
// 			}
// 			daa++
// 			dbb++
// 			size := daa * dbb

// 			maxRect = max(size, maxRect)
// 		}
// 	}
// 	return maxRect
// }

// func d9tests() {
// 	if AddRange([][2]int{{0, 3}, {5, 7}}, 3, 5)[0] != [2]int{0, 7} {
// 		panic("a0")
// 	}
// 	r := AddRange([][2]int{{0, 3}}, 5, 6)
// 	if len(r) != 2 && r[0] != [2]int{0, 3} && r[1] != [2]int{5, 6} {
// 		panic("a1")
// 	}
// 	r = AddRange([][2]int{{5, 7}}, 3, 5)
// 	if len(r) != 1 && r[0] != [2]int{3, 7} {
// 		panic("a2")
// 	}

// 	//  01234567
// 	//  X######X
// 	//  ...x-x..
// 	//  X##X.X#X
// 	r = RemoveRange([][2]int{{0, 7}}, 3, 5)
// 	if len(r) != 2 || r[0] != [2]int{0, 3} || r[1] != [2]int{5, 7} {
// 		panic("r0")
// 	}
// 	//  X####X
// 	//  ...X--
// 	//= X##X
// 	r = RemoveRange([][2]int{{0, 5}}, 3, 5)
// 	if len(r) != 1 || r[0] != [2]int{0, 3} {
// 		fmt.Println(r)
// 		panic("r1")
// 	}

// 	//  X####X
// 	//  ---X..
// 	//= ...X#X
// 	r = RemoveRange([][2]int{{0, 5}}, 0, 3)
// 	if len(r) != 1 || r[0] != [2]int{3, 5} {
// 		fmt.Println(r)
// 		panic("r2")
// 	}
// 	r = RemoveRange([][2]int{{0, 5}}, 0, 5)
// 	if len(r) != 0 {
// 		fmt.Println(r)
// 		panic("r3")
// 	}
// }

// func d9ex2(input D9Input) int {
// 	d9tests()

// 	maxRect := 0
// 	for i, aa := range input {
// 		for _, bb := range input[i+1:] {

// 			aa, bb = [2]int{min(aa[0], bb[0]), min(aa[1], bb[1])}, [2]int{max(aa[0], bb[0]), max(aa[1], bb[1])}

// 			if GreenTilesBrute(input, aa, bb) {

// 				daa := bb[0] - aa[0]
// 				dbb := bb[1] - aa[1]

// 				daa++
// 				dbb++
// 				size := daa * dbb

// 				fmt.Println(aa, bb, size)

// 				maxRect = max(size, maxRect)
// 			}
// 		}
// 	}
// 	return maxRect
// }

// type D9Set map[[2]int]struct{}

// // func HasGreens(sweeps []RangesOnLine, aa, bb [2]int) bool {

// // 	start := -1
// // 	end := -1

// // 	for i, sweep := range sweeps {
// // 		if sweep.y == aa[1] {
// // 			start = i
// // 		}
// // 		if sweep.y == bb[1] {
// // 			end = i
// // 		}
// // 	}
// // 	if start == -1 {
// // 		panic("no start found")
// // 	}
// // 	if end == -1 {
// // 		panic("no end found")
// // 	}
// // 	if start >= end {
// // 		panic("heh")
// // 	}

// // 	ok := true

// // 	for _, sweep := range sweeps[start : end+1] {

// // 		if !HasRange(sweep.ranges, aa[0], bb[0]) {
// // 			ok = false
// // 			break
// // 		}
// // 	}

// // 	// fmt.Println("hg", aa, bb, start, end, ok)
// // 	return ok
// // }

// type RangesOnLine struct {
// 	y      int
// 	ranges [][2]int
// }

// func GreenTilesBrute(input D9Input, aa, bb [2]int) bool {

// 	sweep := make([][2]int, 0)

// 	startedChecking := false

// 	dl, dr := aa[0], bb[0]

// 	if dl > dr || aa[1] > bb[1] {
// 		panic("no")
// 	}

// 	lines := make([][2][2]int, len(input)/2)

// 	nextIsHor := true

// 	for i, point := range input[1:] {

// 		prevPoint := input[i]
// 		prevPoint, point := [2]int{min(prevPoint[0], point[0]), prevPoint[1]}, [2]int{max(prevPoint[0], point[0]), point[1]}

// 		if point[1] == prevPoint[1] {
// 			if !nextIsHor {
// 				panic("no hor")
// 			}
// 			nextIsHor = false
// 			lines[i/2] = [2][2]int{prevPoint, point}
// 		} else if point[0] == prevPoint[0] {
// 			if nextIsHor {
// 				panic("no ver")
// 			}
// 			nextIsHor = true
// 		}
// 	}

// 	slices.SortFunc(lines, func(a, b [2][2]int) int {
// 		ycmp := cmp.Compare(a[0][1], b[0][1])
// 		if ycmp == 0 {
// 			return cmp.Compare(a[0][0], b[0][0])
// 		}
// 		return ycmp
// 	})

// 	for _, line := range lines {

// 		prevPoint := line[0]
// 		point := line[1]

// 		l, r := prevPoint[0], point[0]
// 		y := prevPoint[1]

// 		sweepLine := make([][2]int, len(sweep))
// 		if HasRange(sweep, l, r) {
// 			copy(sweepLine, sweep)
// 			sweep = RemoveRange(sweep, l, r)
// 		} else {
// 			sweep = AddRange(sweep, l, r)
// 			copy(sweepLine, sweep)
// 		}

// 		if y == aa[1] {
// 			startedChecking = true
// 		}
// 		if startedChecking {
// 			if !HasRange(sweepLine, dl, dr) {
// 				return false
// 			}
// 		}
// 		if y == bb[1] {
// 			if !startedChecking {
// 				panic("invalid state?")
// 			}
// 			return true
// 		}

// 	}

// 	panic("unreach")
// }

// func AddRange(sweep [][2]int, l, r int) [][2]int {
// 	// first check if combined?
// 	leftMerge := -1
// 	rightMerge := -1
// 	for i, ra := range sweep {
// 		if ra[1] == l {
// 			leftMerge = i
// 		}
// 		if ra[0] == r {
// 			rightMerge = i
// 		}
// 	}

// 	if leftMerge >= 0 && rightMerge >= 0 {
// 		nl := sweep[leftMerge][0]
// 		nr := sweep[rightMerge][1]
// 		split := max(rightMerge, leftMerge)
// 		sweep = append(sweep[0:split], sweep[split+1:]...)
// 		sweep[leftMerge][0] = nl
// 		sweep[leftMerge][1] = nr
// 	} else if leftMerge >= 0 {
// 		sweep[leftMerge][1] = r
// 	} else if rightMerge >= 0 {
// 		sweep[rightMerge][0] = l
// 	} else {
// 		sweep = append(sweep, [2]int{l, r})
// 	}

// 	return sweep

// }

// func HasRange(sweep [][2]int, l, r int) bool {
// 	for _, ra := range sweep {
// 		if l >= ra[0] && r <= ra[1] {
// 			return true
// 		}
// 	}
// 	return false
// }

// func RemoveRange(sweep [][2]int, l, r int) [][2]int {
// 	containingRange := 0

// 	for i, ra := range sweep {
// 		if l >= ra[0] && r <= ra[1] {
// 			containingRange = i
// 			break
// 		}
// 	}

// 	cL := sweep[containingRange][0]
// 	cR := sweep[containingRange][1]

// 	if cL == l && cR == r {
// 		return append(sweep[0:containingRange], sweep[containingRange+1:]...)
// 	} else if cL == l {
// 		sweep[containingRange][0] = r
// 	} else if cR == r {
// 		sweep[containingRange][1] = l
// 	} else {
// 		sweep[containingRange][1] = l
// 		sweep = append(sweep, [2]int{r, cR})
// 	}

// 	return sweep
// }
