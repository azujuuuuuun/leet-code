package main

func firstUniqChar(s string) int {
	charCounter := map[string]int{}
	for i := 0; i < len(s); i++ {
		charCounter[s[i:i+1]]++
	}
	for i := 0; i < len(s); i++ {
		v, _ := charCounter[s[i:i+1]]
		if v == 1 {
			return i
		}
	}
	return -1
}
