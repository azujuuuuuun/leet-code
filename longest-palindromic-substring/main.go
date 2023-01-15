package main

func longestPalindrome(s string) string {
	reversed := ""
	for i := len(s) - 1; i >= 0; i-- {
		reversed += s[i : i+1]
	}

	ps := ""
	for i := 0; i < len(s); i++ {
		for j := 0; i+j < len(s); j++ {
			ss := s[i : len(s)-j]
			rs := reversed[j : len(reversed)-i]
			if ss == rs && len(ss) > len(ps) {
				ps = ss
			}
		}
	}

	return ps
}
