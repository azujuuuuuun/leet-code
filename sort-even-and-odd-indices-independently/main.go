package main

import (
	"fmt"
	"sort"
)

func sortEvenOdd(nums []int) []int {
	oddNums := []int{}
	evenNums := []int{}
	sorted := []int{}

	for i, n := range nums {
		if i%2 == 0 {
			evenNums = append(evenNums, n)
		} else {
			oddNums = append(oddNums, n)
		}
	}

	sort.Ints(oddNums)
	sort.Ints(evenNums)

	for i := 0; i < len(nums); i++ {
		if i%2 == 0 {
			sorted = append(sorted, evenNums[i/2])
		} else {
			sorted = append(sorted, oddNums[len(oddNums)-1-i/2])
		}
	}

	return sorted
}

func main() {
	fmt.Println(sortEvenOdd([]int{4, 1, 2, 3}))
	fmt.Println(sortEvenOdd([]int{2, 1}))
	fmt.Println(sortEvenOdd([]int{4, 1, 2, 3, 5}))
}
