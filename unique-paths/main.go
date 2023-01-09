package main

import "fmt"

func uniquePaths(m int, n int) int {
	pathCount := [][]int{}
	for i := 0; i < m; i++ {
		pathCount = append(pathCount, []int{})
		for j := 0; j < n; j++ {
			pathCount[i] = append(pathCount[i], 1)
		}
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			pathCount[i][j] = pathCount[i-1][j] + pathCount[i][j-1]
		}
	}

	return pathCount[m-1][n-1]
}

func main() {
	fmt.Println(uniquePaths(3, 7))
	fmt.Println(uniquePaths(3, 2))
	fmt.Println(uniquePaths(100, 100))
}
