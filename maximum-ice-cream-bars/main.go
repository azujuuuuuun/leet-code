package main

import (
	"fmt"
	"sort"
)

func maxIceCream(costs []int, coins int) int {
	var sortedCosts []int
	sortedCosts = append(sortedCosts, costs...)
	sort.Ints(sortedCosts)

	num := 0
	rest := coins
	for _, v := range sortedCosts {
		rest -= v
		if rest >= 0 {
			num++
		} else {
			break
		}
	}

	return num
}

func main() {
	fmt.Println(maxIceCream([]int{1, 3, 2, 4, 1}, 7))
	fmt.Println(maxIceCream([]int{10, 6, 8, 7, 7, 8}, 5))
	fmt.Println(maxIceCream([]int{1, 6, 3, 1, 2, 5}, 20))
}
