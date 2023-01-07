package main

import (
	"fmt"
)

func canCompleteCircuit(gas []int, cost []int) int {
	diffSum := 0
	tank := 0
	si := 0
	for i := 0; i < len(gas); i++ {
		if tank < 0 {
			tank = 0
			si = i
		}
		d := gas[i] - cost[i]
		diffSum += d
		tank += d
	}

	if diffSum < 0 {
		return -1
	}

	return si
}

func main() {
	fmt.Println(canCompleteCircuit([]int{1, 2, 3, 4, 5}, []int{3, 4, 5, 1, 2}))
	fmt.Println(canCompleteCircuit([]int{2, 3, 4}, []int{3, 4, 3}))
	fmt.Println(canCompleteCircuit([]int{5, 1, 2, 3, 4}, []int{2, 3, 4, 5, 1}))
	fmt.Println(canCompleteCircuit([]int{2}, []int{2}))
	fmt.Println(canCompleteCircuit([]int{1, 2, 3, 4, 5, 5, 70}, []int{2, 3, 4, 3, 9, 6, 2}))
}
