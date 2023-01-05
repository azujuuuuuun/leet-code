package main

import "fmt"

func containsDuplicate(nums []int) bool {
	numCounter := map[int]int{}
	for i := 0; i < len(nums); i++ {
		numCounter[nums[i]]++
	}
	for i := 0; i < len(nums); i++ {
		v, _ := numCounter[nums[i]]
		if v > 1 {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(containsDuplicate([]int{1, 2, 3, 1}))
	fmt.Println(containsDuplicate([]int{1, 2, 3, 4}))
	fmt.Println(containsDuplicate([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}))
}
