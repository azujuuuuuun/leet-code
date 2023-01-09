package main

import (
	"fmt"
	"strings"
)

func isPalindrome(s string) bool {
	lowerS := strings.ToLower(s)

	alphaS := ""
	for i := 0; i < len(lowerS); i++ {
		c := lowerS[i : i+1]
		if (c >= "a" && c <= "z") || (c >= "0" && c <= "9") {
			alphaS += c
		}
	}

	palindrome := true
	for i := 0; i < len(alphaS); i++ {
		if alphaS[i] != alphaS[len(alphaS)-1-i] {
			palindrome = false
		}
	}

	return palindrome
}

func main() {
	fmt.Println(isPalindrome("A man, a plan, a canal: Panama"))
	fmt.Println(isPalindrome("race a car"))
	fmt.Println(isPalindrome(" "))
	fmt.Println(isPalindrome("0P"))
}
