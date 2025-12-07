package main

import (
	"cmp"
	"fmt"
	"iter"
	"slices"
	"strconv"
	"strings"
	"time"
)

func main() {

	var input, err = readFile(8)

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	start := time.Now()

	points := loadd8Points(input)

	preprocessTime := time.Now().Sub(start)
	start = time.Now()

	ex1 := d8ex1(points)

	ex1elapsed := time.Now().Sub(start)

	fmt.Printf("Ex 1: %d\n", ex1)

	start = time.Now()

	ex2 := d8ex2(points)

	ex2elapsed := time.Now().Sub(start)

	fmt.Printf("Ex 2: %d\n", ex2)

	fmt.Printf("Preprocess: %s\n", preprocessTime.String())
	fmt.Printf("Ex1 time: %s\n", ex1elapsed.String())
	fmt.Printf("Ex2 time: %s\n", ex2elapsed.String())

}

func loadd8Points(input string) [][3]int {
	lines := strings.Split(input, "\n")

	parsed := make([][3]int, len(lines))

	for i, line := range lines {
		parts := strings.Split(line, ",")
		p0, _ := strconv.Atoi(parts[0])
		p1, _ := strconv.Atoi(parts[1])
		p2, _ := strconv.Atoi(parts[2])
		parsed[i] = [3]int{p0, p1, p2}
	}

	return parsed

}

func d8ex1(points [][3]int) int {
	cluster := d8retrieveCluster(points, 1000)

	buckets := make([]int, len(points))
	for _, v := range cluster {
		buckets[v]++
	}
	slices.Sort(buckets)
	slices.Reverse(buckets)

	return buckets[0] * buckets[1] * buckets[2]
}

func d8retrieveCluster(points [][3]int, size int) map[int]int {

	for a, b := range Test(points) {
		if b[0] == 1000 {
			return a
		}
	}
	panic("Unreachable")
}

func d8ex2(points [][3]int) int {

	for a, b := range Test(points) {
		if len(a) == len(points) {
			return points[b[1]][0] * points[b[2]][0]
		}
	}

	panic("unreachable")
}

func DistSquared(a, b [3]int) int {
	da, db, dc := a[0]-b[0], a[1]-b[1], a[2]-b[2]

	return da*da + db*db + dc*dc
}

func Test(points [][3]int) iter.Seq2[map[int]int, [3]int] {

	distancePairs := D8CalculateClosestPoints(points)

	clusters := make(map[int]int)

	connected := 0

	return func(yield func(a map[int]int, b [3]int) bool) {

		for _, pair := range distancePairs {

			mi := SmallestPath(clusters, pair[0])
			ma := SmallestPath(clusters, pair[1])

			// println("t", connected, ";;", pair[0], pair[1], ") ", mi, ma, "=", min(mi, ma))

			next := min(mi, ma)
			clusters[pair[0]] = next
			clusters[pair[1]] = next

			// fmt.Println(clusters, mi, ma)

			if next == mi {
				connected++
				for k, v := range clusters {
					if v == ma {
						clusters[k] = mi

					}
				}
			} else if next == ma {
				connected++
				for k, v := range clusters {
					if v == mi {
						clusters[k] = ma
					}
				}
			}

			if !yield(clusters, [3]int{connected, pair[0], pair[1]}) {
				return
			}
		}
	}

}

func SmallestPath(cluster map[int]int, start int) int {
	a, _ := cluster[start]
	return min(a, start)
}

// p1,p2,dist
func D8CalculateClosestPoints(points [][3]int) [][3]int {
	pairs := make(map[[2]int]int)

	for i, a := range points {
		for j, b := range points[i+1:] {
			j = i + j + 1

			dist := DistSquared(a, b)
			pairs[[2]int{i, j}] = dist
		}
	}

	distancePairs := make([][3]int, len(pairs)) // p1,p2,dist

	indx := 0
	for k, v := range pairs {
		distancePairs[indx] = [3]int{k[0], k[1], v}
		indx++
	}

	slices.SortFunc(distancePairs, func(a, b [3]int) int {
		return cmp.Compare(a[2], b[2])
	})

	return distancePairs
}
