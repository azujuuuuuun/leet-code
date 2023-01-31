package main

import "fmt"

func longestPalindrome(s string) int {
	charMap := map[string]int{}
	for i := 0; i < len(s); i++ {
		charMap[s[i:i+1]]++
	}

	includeOdd := false
	length := 0
	for _, v := range charMap {
		if v%2 == 0 {
			length += v
		} else {
			length += v - v%2
			includeOdd = true
		}
	}
	if includeOdd {
		length += 1
	}

	return length
}

func main() {
	fmt.Println(longestPalindrome("abccccdd"))
	fmt.Println(longestPalindrome("a"))
	fmt.Println(longestPalindrome("abcccdd"))
	fmt.Println(longestPalindrome("ccc"))
}
