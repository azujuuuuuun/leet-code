package main

import "fmt"

func maxArea(height []int) int {
	left := 0
	right := len(height) - 1
	max := 0

	for left < right {
		area := 0
		width := right - left
		if height[left] < height[right] {
			area = height[left] * width
			left++
		} else {
			area = height[right] * width
			right--
		}
		if area > max {
			max = area
		}
	}

	return max
}

func main() {
	fmt.Println(maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7}))
	fmt.Println(maxArea([]int{1, 1}))
}
