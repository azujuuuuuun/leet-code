package main

import (
	"fmt"
	"sort"
)

type ByXEnd [][]int

func (x ByXEnd) Len() int {
	return len(x)
}

func (x ByXEnd) Swap(i, j int) {
	x[i], x[j] = x[j], x[i]
}

func (x ByXEnd) Less(i, j int) bool {
	return x[i][1] < x[j][1]
}

func findMinArrowShots(points [][]int) int {
	sortedPoints := make([][]int, len(points))
	copy(sortedPoints, points)
	sort.Sort(ByXEnd(sortedPoints))

	currentIndex := 0
	minArrowShots := 1
	for i := 0; i < len(sortedPoints); i++ {
		if sortedPoints[i][0] <= sortedPoints[currentIndex][1] {
			continue
		} else {
			currentIndex = i
			minArrowShots++
		}
	}

	return minArrowShots
}

func main() {
	fmt.Println(findMinArrowShots([][]int{{10, 16}, {2, 8}, {1, 6}, {7, 12}}))
	fmt.Println(findMinArrowShots([][]int{{1, 2}, {3, 4}, {5, 6}, {7, 8}}))
	fmt.Println(findMinArrowShots([][]int{{1, 2}, {2, 3}, {3, 4}, {4, 5}}))
}
