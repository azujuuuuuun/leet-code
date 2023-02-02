package main

import (
	"fmt"
	"strings"
)

func isLessOrder(word1, word2, order string) bool {
	min := len(word1)
	if len(word2) < min {
		min = len(word2)
	}
	for i := 0; i < min; i++ {
		i1 := strings.Index(order, word1[i:i+1])
		i2 := strings.Index(order, word2[i:i+1])
		if i1 < i2 {
			return true
		}
		if i1 > i2 {
			return false
		}
	}
	if len(word1) > len(word2) {
		return false
	}
	return true
}

func isAlienSorted(words []string, order string) bool {
	if len(words) == 1 {
		return true
	}
	for i := 0; i < len(words)-1; i++ {
		if !isLessOrder(words[i], words[i+1], order) {
			return false
		}
	}
	return true
}

func main() {
	fmt.Println(isAlienSorted([]string{"hello", "leetcode"}, "hlabcdefgijkmnopqrstuvwxyz"))
	fmt.Println(isAlienSorted([]string{"word", "world", "row"}, "worldabcefghijkmnpqstuvxyz"))
	fmt.Println(isAlienSorted([]string{"apple", "app"}, "abcdefghijklmnopqrstuvwxyz"))
}
