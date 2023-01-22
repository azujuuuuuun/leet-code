package main

import "fmt"

func isPalindrome(s string) bool {
	for i := 0; i < len(s); i++ {
		if s[i:i+1] != s[len(s)-1-i:len(s)-i] {
			return false
		}
	}
	return true
}

func divide(s string, sub []string, ans *[][]string) {
	if len(s) == 0 {
		*ans = append(*ans, sub)
	}
	for i := 0; i < len(s); i++ {
		if isPalindrome(s[0 : i+1]) {
			newSub := append([]string{}, sub...)
			newSub = append(newSub, s[0:i+1])
			divide(s[i+1:], newSub, ans)
		}
	}
}

func partition(s string) [][]string {
	ans := [][]string{}

	divide(s, []string{}, &ans)

	return ans
}

func main() {
	fmt.Println(partition("aab"))
	fmt.Println(partition("a"))
}
