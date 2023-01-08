package main

import "fmt"

func majorityElement(nums []int) int {
	numCounter := map[int]int{}
	maxCount := 0
	majorityNum := 0

	for _, n := range nums {
		numCounter[n]++
		if numCounter[n] >= maxCount {
			maxCount = numCounter[n]
			majorityNum = n
		}
	}

	return majorityNum
}

func main() {
	fmt.Println(majorityElement([]int{3, 2, 3}))
	fmt.Println(majorityElement([]int{2, 2, 1, 1, 1, 2, 2}))
}
