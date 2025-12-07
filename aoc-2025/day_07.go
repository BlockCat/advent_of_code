package main

import (
	"fmt"
	"strings"
)

func main() {

	var input, err = readFile(7)
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	var grid = InitTGrid(input)

	fmt.Printf("Ex 1: %d\n", d7ex1(grid))
	grid = InitTGrid(input)
	fmt.Printf("Ex 2: %d\n", d7ex2(grid))
}

func d7ex1(grid TGrid) int {
	split := 0
	queue := make([][2]int, 10)
	queue = append(queue, [2]int{grid.startX, grid.startY})

	for len(queue) > 0 {
		pos := queue[0]
		queue = queue[1:]

		nextX, nextY := pos[0], pos[1]+1
		nextCell, err := grid.Has(nextX, nextY)
		if err != nil {
			continue
		}
		if nextCell == Empty {
			queue = append(queue, [2]int{nextX, nextY})
			grid.Set(nextX, nextY, Beam)
		} else if nextCell == Splitter {
			split++
			l, el := grid.Has(nextX-1, nextY)
			r, er := grid.Has(nextX+1, nextY)

			if l == Empty && el == nil {
				queue = append(queue, [2]int{nextX - 1, nextY})
				grid.Set(nextX-1, nextY, Beam)
			}

			if r == Empty && er == nil {
				queue = append(queue, [2]int{nextX + 1, nextY})
				grid.Set(nextX+1, nextY, Beam)
			}
		}

	}

	return split
}

func d7ex2(grid TGrid) int {

	cache := make([]int, len(grid.cells))

	return d7ex2s(grid, grid.startX, grid.startY, cache)
}

func d7ex2s(grid TGrid, x, y int, cache []int) int {
	for true {
		x, y = x, y+1
		// println(x, y)
		nextCell, err := grid.Has(x, y)
		if err != nil {
			return 1
		}

		if nextCell == Empty {
			continue
		} else if nextCell == Splitter {

			l, el := grid.Has(x-1, y)
			r, er := grid.Has(x+1, y)

			dims := 0

			if l == Empty && el == nil {
				index := (x - 1) + y*grid.width
				cached := cache[index]
				dl := 0
				if cached == 0 {
					dl = d7ex2s(grid, x-1, y, cache)
					cache[index] = dl
				} else {
					dl = cached
				}
				dims += dl
			}

			if r == Empty && er == nil {
				index := (x + 1) + y*grid.width
				cached := cache[index]
				dl := 0
				if cached == 0 {
					dl = d7ex2s(grid, x+1, y, cache)
					cache[index] = dl
				} else {
					dl = cached
				}
				dims += dl
			}
			return dims
		}

	}

	panic("unreachable")
}

type GridCell int

const (
	Empty GridCell = iota
	Start
	Splitter
	Beam
)

type TGrid struct {
	cells  []GridCell
	width  int
	height int
	startX int
	startY int
}

func InitTGrid(input string) TGrid {
	lines := strings.Split(input, "\n")

	height := len(lines)
	width := len(lines[0])

	cells := make([]GridCell, height*width)

	sx, sy := 0, 0

	for y := range lines {
		for x, v := range lines[y] {
			var index = x + y*width
			switch v {
			case 'S':
				cells[index] = GridCell(Start)
				sx, sy = x, y

			case '.':
				cells[index] = GridCell(Empty)
			case '^':
				cells[index] = GridCell(Splitter)
			default:
				panic("Unrecognized: " + string(v))
			}
		}
	}

	return TGrid{
		width:  width,
		height: height,
		cells:  cells,
		startX: sx,
		startY: sy,
	}
}

func (g *TGrid) Has(x, y int) (GridCell, error) {

	if x < 0 || x >= g.width || y < 0 || y >= g.height {
		return Empty, fmt.Errorf("Out of bounds: (%d,%d) out of [%d,%d]", x, y, g.width, g.height)
	}
	index := x + y*g.width
	return g.cells[index], nil

}
func (g *TGrid) Set(x, y int, v GridCell) error {

	if x < 0 || x >= g.width || y < 0 || y >= g.height {
		return fmt.Errorf("Out of bounds: (%d,%d) out of [%d,%d]", x, y, g.width, g.height)
	}
	index := x + y*g.width
	g.cells[index] = v

	return nil

}
