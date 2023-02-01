package main

import (
	"fmt"
	"strings"
)

func gcdOfStrings(str1 string, str2 string) string {
	shortS := ""
	longS := ""
	if len(str1) < len(str2) {
		shortS = str1
		longS = str2
	} else {
		shortS = str2
		longS = str1
	}

	gcd := ""
	for i := 0; i < len(shortS); i++ {
		ss := shortS[0 : i+1]
		if strings.ReplaceAll(shortS, ss, "") == "" && strings.ReplaceAll(longS, ss, "") == "" {
			gcd = ss
		}
	}

	return gcd
}

func main() {
	fmt.Println(gcdOfStrings("ABCABC", "ABC"))
	fmt.Println(gcdOfStrings("ABABAB", "AB"))
	fmt.Println(gcdOfStrings("LEET", "CODE"))
	fmt.Println(gcdOfStrings("TAUXXTAUXXTAUXXTAUXXTAUXX", "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX"))
}
