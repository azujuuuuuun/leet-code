package main

import "fmt"

func findAnagrams(s string, p string) []int {
	sCharCount := [][]int{}
	for i := 0; i < 26; i++ {
		sCharCount = append(sCharCount, []int{})
		for j := 0; j <= len(s); j++ {
			sCharCount[i] = append(sCharCount[i], 0)
		}
	}
	for i := 1; i <= len(s); i++ {
		for j := 0; j < 26; j++ {
			sCharCount[j][i] = sCharCount[j][i-1]
		}
		charIndex := []rune(s[i-1 : i])[0] - rune('a')
		sCharCount[charIndex][i]++
	}

	pCharCount := map[string]int{}
	for i := 0; i < len(p); i++ {
		pCharCount[p[i:i+1]]++
	}

	indices := []int{}
	for i := 0; i <= len(s)-len(p); i++ {
		ok := true
		for k, v := range pCharCount {
			charIndex := []rune(k)[0] - rune('a')
			if v != sCharCount[charIndex][i+len(p)]-sCharCount[charIndex][i] {
				ok = false
			}
		}
		if ok {
			indices = append(indices, i)
		}
	}

	return indices
}

func main() {
	fmt.Println(findAnagrams("cbaebabacd", "abc"))
	fmt.Println(findAnagrams("abab", "ab"))
}
