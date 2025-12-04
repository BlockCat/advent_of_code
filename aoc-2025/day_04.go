package main

import (
	"fmt"
	"math/bits"
	"strings"
	"time"
)

func main() {

	var input, err = readFile(4)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var ex1 = 0
	var ex2 = 0

	start := time.Now()

	grid := InitGrid(input)

	fmt.Printf("Time to load stuff: %d us\n", time.Now().Sub(start).Microseconds())

	start = time.Now()

	for y := range grid.height {
		for x := range grid.width {
			if grid.Has(x, y) && grid.NeighbourCount(x, y) < 4 {
				ex1++
			}
		}
	}

	fmt.Printf("Time to ex1 stuff: %d us \n", time.Now().Sub(start).Microseconds())
	start = time.Now()

	for {
		removed := 0
		for y := range grid.height {
			for x := range grid.width {
				if grid.Has(x, y) && grid.NeighbourCount(x, y) < 4 {
					ex2++
					removed++
					grid.Remove(x, y)
				}
			}
		}

		if removed == 0 {
			break
		}
	}

	fmt.Printf("Time to ex2 stuff: %d us\n ", time.Now().Sub(start).Microseconds())

	fmt.Printf("Ex 1: %d\n", ex1)
	fmt.Printf("Ex 2: %d\n", ex2)
	// fmt.Printf("Ex 2: %d joltage\n", sum2)

}

type Grid struct {
	grid       []bool
	neighbours []uint8
	width      int
	height     int
}

func InitGrid(input string) *Grid {
	lines := strings.Split(strings.TrimSpace(input), "\n")

	// we use padding of false
	height, width := len(lines)+2, len(lines[0])+2

	grid := make([]bool, width*height)
	neighbours := make([]uint8, width*height)

	cg := Grid{
		grid: grid, neighbours: neighbours,
		width: width - 2, height: height - 2,
	}

	for y, hor := range lines {
		for x, v := range hor {
			if v == '@' {
				cg.Add(x, y)
			}
		}
	}
	return &cg
}

func (g *Grid) Add(x int, y int) {
	width := g.width + 2
	index := (x + 1) + (y+1)*width

	g.grid[index] = true

	g.neighbours[index-1-width] |= 0b1000_0000
	g.neighbours[index-1] |= 0b0100_0000
	g.neighbours[index-1+width] |= 0b0010_0000
	g.neighbours[index-width] |= 0b0001_0000
	g.neighbours[index+width] |= 0b0000_1000
	g.neighbours[index+1-width] |= 0b0000_0100
	g.neighbours[index+1] |= 0b0000_0010
	g.neighbours[index+1+width] |= 0b0000_0001

}

func (g *Grid) Remove(x int, y int) {
	width := g.width + 2
	index := (x + 1) + (y+1)*width

	g.grid[index] = false

	g.neighbours[index-1-width] &= 0b0111_1111
	g.neighbours[index-1] &= 0b1011_1111
	g.neighbours[index-1+width] &= 0b1101_1111
	g.neighbours[index-width] &= 0b1110_1111
	g.neighbours[index+width] &= 0b1111_0111
	g.neighbours[index+1-width] &= 0b1111_1011
	g.neighbours[index+1] &= 0b1111_1101
	g.neighbours[index+1+width] &= 0b1111_1110
}

func (g *Grid) NeighbourCount(x int, y int) int {
	return bits.OnesCount8(g.neighbours[(x+1)+(y+1)*(g.width+2)])
}

func (g *Grid) Has(x int, y int) bool {
	return g.grid[(x+1)+(y+1)*(g.width+2)]
}
