package main

import (
	"fmt"
	"math"
)

func titleToNumber(columnTitle string) int {
	colNum := 0
	n := len(columnTitle)

	for i, c := range columnTitle {
		num := int(c - 64)
		colNum += int(math.Pow(26, float64(n-1-i))) * num
	}

	return colNum
}

func main() {
	fmt.Println(titleToNumber("A"))
	fmt.Println(titleToNumber("AB"))
	fmt.Println(titleToNumber("ZY"))
}
