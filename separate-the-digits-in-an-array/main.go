package main

import "fmt"

func separateDigits(nums []int) []int {
	digitsOfNums := []int{}

	for i := 0; i < len(nums); i++ {
		digits := []int{}
		num := nums[i]

		for num >= 1 {
			d := num % 10
			num /= 10
			digits = append(digits, d)
		}

		for j := len(digits) - 1; j >= 0; j-- {
			digitsOfNums = append(digitsOfNums, digits[j])
		}
	}

	return digitsOfNums
}

func main() {
	fmt.Println(separateDigits([]int{13, 25, 83, 77}))
	fmt.Println(separateDigits([]int{7, 1, 3, 9}))
}
