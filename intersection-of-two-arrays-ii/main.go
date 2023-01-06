package main

import (
	"fmt"
	"sort"
)

func intersect(nums1 []int, nums2 []int) []int {
	var short []int
	var long []int
	if len(nums1) <= len(nums2) {
		short = append(short, nums1...)
		long = append(long, nums2...)
	} else {
		short = append(short, nums2...)
		long = append(long, nums1...)
	}

	sort.Ints(short)
	sort.Ints(long)

	si, li := 0, 0
	var intersection []int
	for {
		if si == len(short) || li == len(long) {
			break
		}
		if short[si] == long[li] {
			intersection = append(intersection, short[si])
			si++
			li++
		} else if short[si] < long[li] {
			si++
		} else {
			li++
		}
	}

	return intersection
}

func main() {
	fmt.Println(intersect([]int{1, 2, 2, 1}, []int{2, 2}))
	fmt.Println(intersect([]int{4, 9, 5}, []int{9, 4, 9, 8, 4}))
}
