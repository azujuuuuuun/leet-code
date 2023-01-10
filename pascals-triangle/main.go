package main

import "fmt"

func generate(numRows int) [][]int {
	triangle := [][]int{{1}}

	if numRows == 1 {
		return triangle
	}

	triangle = append(triangle, []int{1, 1})
	if numRows == 2 {
		return triangle
	}

	for i := 3; i <= numRows; i++ {
		row := []int{}
		for j := 0; j < len(triangle[i-2]); j++ {
			if j == 0 {
				row = append(row, triangle[i-2][j])
			}
			if j == len(triangle[i-2])-1 {
				row = append(row, triangle[i-2][j])
				break
			}
			row = append(row, triangle[i-2][j]+triangle[i-2][j+1])
		}
		triangle = append(triangle, row)
	}

	return triangle
}

func main() {
	fmt.Println(generate(1))
	fmt.Println(generate(2))
	fmt.Println(generate(3))
	fmt.Println(generate(4))
	fmt.Println(generate(5))
}
