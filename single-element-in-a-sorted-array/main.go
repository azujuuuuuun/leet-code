package main

func singleNonDuplicate(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}

	for i := 0; i < len(nums); i++ {
		if i == 0 {
			if nums[i] != nums[i+1] {
				return nums[i]
			}
		} else if i == len(nums)-1 {
			if nums[i] != nums[i-1] {
				return nums[i]
			}
		} else {
			if nums[i] != nums[i-1] && nums[i] != nums[i+1] {
				return nums[i]
			}
		}
	}
	return -1
}
