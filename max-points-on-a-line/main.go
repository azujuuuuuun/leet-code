package main

import "fmt"

type Line struct {
	x  int
	y  int
	dx int
	dy int
	c  int
}

func maxPoints(points [][]int) int {
	if len(points) == 1 || len(points) == 2 {
		return len(points)
	}

	lines := []Line{}
	for i := 0; i < len(points)-1; i++ {
		for j := i + 1; j < len(points); j++ {
			dx := points[j][0] - points[i][0]
			dy := points[j][1] - points[i][1]
			c := -1
			if dx == 0 {
				c = points[i][0]
			}
			lines = append(lines, Line{x: points[i][0], y: points[i][1], dx: dx, dy: dy, c: c})
		}
	}

	maxPoints := 0
	for i := 0; i < len(lines); i++ {
		count := 0
		x, y, dx, dy, c := lines[i].x, lines[i].y, lines[i].dx, lines[i].dy, lines[i].c
		for j := 0; j < len(points); j++ {
			if c != -1 && points[j][0] == c {
				count++
			} else if dx*points[j][1] == dy*points[j][0]+dx*y-dy*x {
				count++
			}
		}
		if count >= maxPoints {
			maxPoints = count
		}
	}

	return maxPoints
}

func main() {
	fmt.Println(maxPoints([][]int{{1, 1}, {2, 2}, {3, 3}}))
	fmt.Println(maxPoints([][]int{{1, 1}, {3, 2}, {5, 3}, {4, 1}, {2, 3}, {1, 4}}))
	fmt.Println(maxPoints([][]int{{0, 1}, {0, 0}}))
	fmt.Println(maxPoints([][]int{{3, 10}, {0, 2}}))
	fmt.Println(maxPoints([][]int{{-6, -1}, {3, 1}, {12, 3}}))
}
