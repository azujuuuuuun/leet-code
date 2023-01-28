package main

import "fmt"

func removeDuplicates(nums []int) int {
	k := 1
	num := nums[0]

	for i := 1; i < len(nums); i++ {
		if nums[i] != num {
			nums[k], num = nums[i], nums[i]
			k++
			continue
		}
	}

	return k
}

func main() {
	fmt.Println(removeDuplicates([]int{1, 1, 2}))
	fmt.Println(removeDuplicates([]int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}))
	fmt.Println(removeDuplicates([]int{0, 1, 2, 3, 4}))
}
