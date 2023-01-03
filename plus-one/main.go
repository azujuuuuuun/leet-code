package main

import "fmt"

func plusOne(digits []int) []int {
	ans := append([]int{}, digits...)

	carry := 1
	for i := len(ans) - 1; i >= 0; i-- {
		d := ans[i] + carry
		ans[i] = d % 10
		carry = d / 10

		if i == 0 && carry == 1 {
			ans = append([]int{1}, ans...)
		}
	}

	return ans
}

func main() {
	fmt.Println(plusOne([]int{1, 2, 3}))
	fmt.Println(plusOne([]int{4, 3, 2, 1}))
	fmt.Println(plusOne([]int{9}))
	fmt.Println(plusOne([]int{9, 9}))
}
