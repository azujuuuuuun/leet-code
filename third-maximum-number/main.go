package main

import (
	"fmt"
	"sort"
)

func thirdMax(nums []int) int {
	numMap := map[int]int{}

	for i := 0; i < len(nums); i++ {
		numMap[nums[i]]++
	}

	distinctNums := []int{}
	for k := range numMap {
		distinctNums = append(distinctNums, k)
	}

	sort.Ints(distinctNums)

	if len(distinctNums) <= 2 {
		return distinctNums[len(distinctNums)-1]
	}
	return distinctNums[len(distinctNums)-3]
}

func main() {
	fmt.Println(thirdMax([]int{3, 2, 1}))
	fmt.Println(thirdMax([]int{1, 2}))
	fmt.Println(thirdMax([]int{2, 2, 3, 1}))
}
