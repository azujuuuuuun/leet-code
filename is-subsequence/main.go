package main

import "fmt"

func isSubsequence(s string, t string) bool {
	si := 0
	ti := 0
	isSubs := false

	for {
		if si >= len(s) && ti >= len(t) {
			isSubs = true
			break
		}

		if si >= len(s) {
			isSubs = true
			break
		}

		if ti >= len(t) {
			break
		}

		if s[si:si+1] == t[ti:ti+1] {
			si++
			ti++
			continue
		}

		ti++
	}

	return isSubs
}

func main() {
	fmt.Println(isSubsequence("abc", "ahbgdc"))
	fmt.Println(isSubsequence("axc", "ahbgdc"))
}
