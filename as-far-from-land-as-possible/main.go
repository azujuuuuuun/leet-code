package main

import (
	"fmt"
)

func maxDistance(grid [][]int) int {
	visited := [][]bool{}
	queue := [][]int{}

	for i := 0; i < len(grid); i++ {
		visited = append(visited, []bool{})
		for j := 0; j < len(grid[i]); j++ {
			visited[i] = append(visited[i], false)
			cell := grid[i][j]
			if cell == 1 {
				visited[i][j] = true
				queue = append(queue, []int{i, j})
			}
		}
	}

	dist := -1
	dict := [][]int{{0, -1}, {1, 0}, {0, 1}, {-1, 0}}
	for len(queue) != 0 {
		qSize := len(queue)

		for qSize != 0 {
			cell := queue[0]
			queue = queue[1:]
			qSize--

			for _, d := range dict {
				x := cell[0] + d[0]
				y := cell[1] + d[1]
				if x >= 0 && x < len(grid) && y >= 0 && y < len(grid) && !visited[x][y] {
					visited[x][y] = true
					queue = append(queue, []int{x, y})
				}
			}
		}

		dist++
	}

	if dist == 0 {
		return -1
	}
	return dist
}

func main() {
	fmt.Println(maxDistance([][]int{{1, 0, 1}, {0, 0, 0}, {1, 0, 1}}))
	fmt.Println(maxDistance([][]int{{1, 0, 0}, {0, 0, 0}, {0, 0, 0}}))
	fmt.Println(maxDistance([][]int{{0, 0, 0}, {0, 0, 0}, {0, 0, 0}, {0, 0, 0}}))
}
