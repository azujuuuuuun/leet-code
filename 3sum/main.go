package main

import (
	"fmt"
	"sort"
)

func reduceNums(nums []int) []int {
	numCount := map[int]int{}
	for _, n := range nums {
		numCount[n]++
	}

	reducedNums := []int{}
	for n, c := range numCount {
		max := 3
		if c < max {
			max = c
		}
		for i := 0; i < max; i++ {
			reducedNums = append(reducedNums, n)
		}
	}

	return reducedNums
}

func threeSum(nums []int) [][]int {
	reducedNums := reduceNums(nums)

	numList := map[int][]int{}
	for i, n := range reducedNums {
		_, ok := numList[n]
		if ok {
			numList[n] = append(numList[n], i)
		} else {
			numList[n] = []int{i}
		}
	}

	triples := [][]int{}
	for i := 0; i < len(reducedNums)-1; i++ {
		for j := i + 1; j < len(reducedNums); j++ {
			numK := -reducedNums[i] - reducedNums[j]
			list, ok := numList[numK]
			if !ok {
				continue
			}
			for li := 0; li < len(list); li++ {
				if i != j && i != list[li] && j != list[li] && list[li] > i && list[li] > j {
					triple := []int{reducedNums[i], reducedNums[j], numK}
					sort.Ints(triple)

					includes := false
					for _, t := range triples {
						if t[0] == triple[0] && t[1] == triple[1] && t[2] == triple[2] {
							includes = true
						}
					}
					if !includes {
						triples = append(triples, triple)
					}
				}
			}
		}
	}

	return triples
}

func main() {
	fmt.Println(threeSum([]int{-1, 0, 1, 2, -1, -4}))
	fmt.Println(threeSum([]int{0, 1, 1}))
	fmt.Println(threeSum([]int{0, 0, 0}))
}
