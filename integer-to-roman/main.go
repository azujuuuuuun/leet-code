package main

import "fmt"

type Pair struct {
	value  int
	symbol string
}

func intToRoman(num int) string {
	pairs := []Pair{{1000, "M"}, {900, "CM"}, {500, "D"}, {400, "CD"}, {100, "C"}, {90, "XC"}, {50, "L"}, {40, "XL"}, {10, "X"}, {9, "IX"}, {5, "V"}, {4, "IV"}, {1, "I"}}

	n := num
	roman := ""
	pairIndex := 0

	for n != 0 {
		if n >= pairs[pairIndex].value {
			n -= pairs[pairIndex].value
			roman += pairs[pairIndex].symbol
		} else {
			pairIndex++
		}
	}

	return roman
}

func main() {
	fmt.Println(intToRoman(3))
	fmt.Println(intToRoman(58))
	fmt.Println(intToRoman(1994))
}
