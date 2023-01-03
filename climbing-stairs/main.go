package main

import "fmt"

func countStepWays(stepWaysMap map[int]int, n int) int {
	v, ok := stepWaysMap[n]
	if ok {
		return v
	}
	sum := countStepWays(stepWaysMap, n-1) + countStepWays(stepWaysMap, n-2)
	stepWaysMap[n] = sum
	return sum
}

func climbStairs(n int) int {
	stepWaysMap := map[int]int{1: 1, 2: 2}
	return countStepWays(stepWaysMap, n)
}

func main() {
	fmt.Println(climbStairs(2))
	fmt.Println(climbStairs(3))
	fmt.Println(climbStairs(45))
}
