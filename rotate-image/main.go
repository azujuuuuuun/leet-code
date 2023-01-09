package main

import "fmt"

func rotate(matrix [][]int) {
	tmp := []int{}
	for _, r := range matrix {
		tmp = append(tmp, r...)
	}

	n := len(matrix)
	for i, r := range matrix {
		for j := range r {
			matrix[j][n-1-i] = tmp[n*i+j]
		}
	}
}

func main() {
	m1 := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
	rotate(m1)
	fmt.Println(m1)

	m2 := [][]int{{5, 1, 9, 11}, {2, 4, 8, 10}, {13, 3, 6, 7}, {15, 14, 12, 16}}
	rotate(m2)
	fmt.Println(m2)
}
