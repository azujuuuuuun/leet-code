package main

import "fmt"

func findKthPositive(arr []int, k int) int {
	for i := 0; i < len(arr); i++ {
		if i == 0 {
			if arr[i] == 1 {
				continue
			}
			if k < arr[i] {
				return k
			}
			k = k - (arr[i] - 1)
			continue
		}

		if arr[i] != arr[i-1]+1 {
			if k <= arr[i]-arr[i-1]-1 {
				return arr[i-1] + k
			}
			k = k - (arr[i] - arr[i-1] - 1)
		}
	}

	if k > 0 {
		return arr[len(arr)-1] + k
	}

	return -1
}

func main() {
	fmt.Println(findKthPositive([]int{2, 3, 4, 7, 11}, 5))
	fmt.Println(findKthPositive([]int{1, 2, 3, 4, 5, 10}, 5))
	// fmt.Println(findKthPositive([]int{1, 2, 3, 4}, 2))
}
