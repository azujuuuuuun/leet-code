package main

import "fmt"

func alternateDigitSum(n int) int {
	digits := []int{}
	for n > 0 {
		digits = append(digits, n%10)
		n /= 10
	}

	sum := 0
	for i := 0; i < len(digits); i++ {
		if i%2 == 0 {
			sum += digits[len(digits)-1-i]
		} else {
			sum -= digits[len(digits)-1-i]
		}
	}

	return sum
}

func main() {
	fmt.Println(alternateDigitSum(521))
	fmt.Println(alternateDigitSum(111))
	fmt.Println(alternateDigitSum(886996))
}
