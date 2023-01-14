package main

import (
	"fmt"
	"strings"
)

func lengthOfLastWord(s string) int {
	trimmed := strings.Trim(s, " ")
	count := 0

	for i := len(trimmed) - 1; i >= 0; i-- {
		c := trimmed[i : i+1]
		if c != " " {
			count++
		}
		if c == " " {
			break
		}
	}

	return count
}

func main() {
	fmt.Println(lengthOfLastWord("Hello World"))
	fmt.Println(lengthOfLastWord("   fly me   to   the moon  "))
	fmt.Println(lengthOfLastWord("luffy is still joyboy"))
	fmt.Println(lengthOfLastWord("Hello"))
	fmt.Println(lengthOfLastWord(" "))
}
