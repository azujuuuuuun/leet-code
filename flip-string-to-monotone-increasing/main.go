package main

import (
	"fmt"
	"math"
)

func minFlipsMonoIncr(s string) int {
	num, ans := 0, 0
	for i := 0; i < len(s); i++ {
		if s[i:i+1] == string('0') {
			ans = int(math.Min(float64(num), float64(ans+1)))
		} else {
			num++
		}
	}
	return ans
}

func main() {
	fmt.Println(minFlipsMonoIncr("00110"), 1)
	fmt.Println(minFlipsMonoIncr("010110"), 2)
	fmt.Println(minFlipsMonoIncr("00011000"), 2)
}
