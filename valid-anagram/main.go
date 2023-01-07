package main

import "fmt"

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	scCounter := map[string]int{}
	tcCounter := map[string]int{}
	for i := 0; i < len(s); i++ {
		scCounter[s[i:i+1]]++
		tcCounter[t[i:i+1]]++
	}

	valid := true
	for k, v := range scCounter {
		count, ok := tcCounter[k]
		if !ok {
			valid = false
		}
		if count != v {
			valid = false
		}
	}

	return valid
}

func main() {
	fmt.Println(isAnagram("anagram", "nagaram"))
	fmt.Println(isAnagram("rat", "car"))
}
