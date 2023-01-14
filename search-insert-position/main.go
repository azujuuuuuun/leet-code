package main

import "fmt"

func searchInsert(nums []int, target int) int {
	left := 0
	right := len(nums)
	index := 0

	for left <= right {
		middle := (left + right) / 2
		if target == nums[middle] {
			index = middle
			break
		}
		if target < nums[middle] {
			if middle == 0 {
				index = 0
				break
			} else {
				if target > nums[middle-1] {
					index = middle
					break
				}
			}
			right = middle
		}
		if target > nums[middle] {
			if middle == len(nums)-1 {
				index = len(nums)
				break
			} else {
				if target < nums[middle+1] {
					index = middle + 1
					break
				}
			}
			left = middle
		}
	}

	return index
}

func main() {
	fmt.Println(searchInsert([]int{1}, 0), 0)
	fmt.Println(searchInsert([]int{1}, 1), 0)
	fmt.Println(searchInsert([]int{1}, 2), 1)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 5), 2)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 2), 1)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 0), 0)
	fmt.Println(searchInsert([]int{1, 3, 5, 6}, 7), 4)
	fmt.Println(searchInsert([]int{1, 3, 5}, 4), 2)
}
