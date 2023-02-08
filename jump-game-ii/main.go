package main

import (
	"fmt"
	"math"
)

func jump(nums []int) int {
	minJumps := []int{}
	for i := 0; i < len(nums); i++ {
		minJumps = append(minJumps, math.MaxInt)
	}
	minJumps[0] = 0

	for i := 0; i < len(nums); i++ {
		n := nums[i]
		for j := 1; j <= n; j++ {
			if i+j >= len(nums) {
				break
			}
			if minJumps[i]+1 < minJumps[i+j] {
				minJumps[i+j] = minJumps[i] + 1
			}
		}
	}

	return minJumps[len(nums)-1]
}

func main() {
	fmt.Println(jump([]int{2, 3, 1, 1, 4}))
	fmt.Println(jump([]int{2, 3, 0, 1, 4}))
}
