package main

import "fmt"

func rec(n int, memo *map[int]int) int {
	v, ok := (*memo)[n]
	if ok {
		return v
	}
	num := rec(n-1, memo) + rec(n-2, memo) + rec(n-3, memo)
	(*memo)[n] = num
	return num
}

func tribonacci(n int) int {
	memo := map[int]int{}
	memo[0] = 0
	memo[1] = 1
	memo[2] = 1
	return rec(n, &memo)
}

func main() {
	fmt.Println(tribonacci(4))
	fmt.Println(tribonacci(25))
}
