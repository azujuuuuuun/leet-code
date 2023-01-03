package main

func twoSum(nums []int, target int) []int {
	i1 := 0
	i2 := 0
	found := false
	for i := 0; i < len(nums); i++ {
		if found {
			break
		}
		for j := 0; j < len(nums); j++ {
			if i == j {
				continue
			}
			if nums[i]+nums[j] == target {
				i1 = i
				i2 = j
				found = true
				break
			}
		}
	}
	return []int{i1, i2}
}
