package main

import "fmt"

func lengthOfLongestSubstring(s string) int {
	if len(s) == 0 {
		return 0
	}

	length := 1
	for i := 0; i < len(s)-1; i++ {
		charCounter := map[string]int{}
		charCounter[s[i:i+1]]++
		count := 1
		for j := i + 1; j < len(s); j++ {
			c := charCounter[s[j:j+1]]
			if c >= 1 {
				break
			}
			charCounter[s[j:j+1]]++
			count++
			if count > length {
				length = count
			}
		}
	}
	return length
}

func main() {
	fmt.Println(lengthOfLongestSubstring("abcabcbb"))
	fmt.Println(lengthOfLongestSubstring("bbbbb"))
	fmt.Println(lengthOfLongestSubstring("pwwkew"))
	fmt.Println(lengthOfLongestSubstring(""))
}
