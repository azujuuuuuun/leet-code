package main

import "fmt"

func moveZeroes(nums []int) {
	for i := 0; i < len(nums); i++ {
		for j := 0; j < len(nums); j++ {
			if j == len(nums)-1 {
				continue
			}
			if nums[j] == 0 {
				nums[j], nums[j+1] = nums[j+1], nums[j]
			}
		}
	}
}

func main() {
	nums1 := []int{0, 1, 0, 3, 12}
	moveZeroes(nums1)
	fmt.Println(nums1)

	nums2 := []int{0}
	moveZeroes(nums2)
	fmt.Println(nums2)
}
