package main

import (
	"fmt"
	"math"
)

func thirdMax(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	if len(nums) == 2 {
		if nums[0] >= nums[1] {
			return nums[0]
		} else {
			return nums[1]
		}
	}

	first, second, third := math.MinInt, math.MinInt, math.MinInt
	for i := 0; i < len(nums); i++ {
		if nums[i] == first || nums[i] == second || nums[i] == third {
			continue
		}
		if nums[i] > first {
			first, second, third = nums[i], first, second
			continue
		}
		if nums[i] > second {
			second, third = nums[i], second
			continue
		}
		if nums[i] > third {
			third = nums[i]
		}
	}

	if first == second || second == third {
		return first
	}
	if first == math.MinInt || second == math.MinInt || third == math.MinInt {
		return first
	}
	return third
}

func main() {
	fmt.Println(thirdMax([]int{3, 2, 1}))
	fmt.Println(thirdMax([]int{1, 2}))
	fmt.Println(thirdMax([]int{2, 2, 3, 1}))
}
