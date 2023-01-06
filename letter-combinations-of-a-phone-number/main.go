package main

import "fmt"

func letterCombinations(digits string) []string {
	if len(digits) == 0 {
		return []string{}
	}

	digitToLetterMap := map[string][]string{
		"2": {"a", "b", "c"},
		"3": {"d", "e", "f"},
		"4": {"g", "h", "i"},
		"5": {"j", "k", "l"},
		"6": {"m", "n", "o"},
		"7": {"p", "q", "r", "s"},
		"8": {"t", "u", "v"},
		"9": {"w", "x", "y", "z"},
	}

	combinations := []string{""}
	for i := 0; i < len(digits); i++ {
		d := digits[i : i+1]
		l := digitToLetterMap[d]
		tmp := []string{}
		for j := 0; j < len(combinations); j++ {
			for k := 0; k < len(l); k++ {
				tmp = append(tmp, combinations[j]+l[k])
			}
		}
		combinations = tmp
	}

	return combinations
}

func main() {
	fmt.Println(letterCombinations("23"))
	fmt.Println(letterCombinations(""))
	fmt.Println(letterCombinations("2"))
}
