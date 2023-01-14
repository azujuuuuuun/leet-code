package main

import (
	"fmt"
	"sort"
)

func smallestEquivalentString(s1 string, s2 string, baseStr string) string {
	groups := [][]string{}
	for i := 0; i < len(s1); i++ {
		c1 := s1[i : i+1]
		c2 := s2[i : i+1]
		groupIndex1 := -1
		groupIndex2 := -1
		for j := 0; j < len(groups); j++ {
			for k := 0; k < len(groups[j]); k++ {
				if c1 == groups[j][k] {
					groupIndex1 = j
				}
				if c2 == groups[j][k] {
					groupIndex2 = j
				}
			}
			if groupIndex1 != -1 && groupIndex2 != -1 {
				break
			}
		}
		if groupIndex1 == -1 && groupIndex2 == -1 {
			if c1 == c2 {
				groups = append(groups, []string{c1})
			} else {
				groups = append(groups, []string{c1, c2})
			}
			continue
		}
		if groupIndex1 != -1 && groupIndex2 == -1 {
			groups[groupIndex1] = append(groups[groupIndex1], c2)
			continue
		}
		if groupIndex1 == -1 && groupIndex2 != -1 {
			groups[groupIndex2] = append(groups[groupIndex2], c1)
			continue
		}
		if groupIndex1 != -1 && groupIndex2 != -1 {
			if groupIndex1 != groupIndex2 {
				group := []string{}
				group = append(group, groups[groupIndex1]...)
				group = append(group, groups[groupIndex2]...)
				groups = append(groups, group)
				if groupIndex1 < groupIndex2 {
					groups = append(groups[:groupIndex1], groups[groupIndex1+1:]...)
					groups = append(groups[:groupIndex2-1], groups[groupIndex2:]...)
				} else {
					groups = append(groups[:groupIndex2], groups[groupIndex2+1:]...)
					groups = append(groups[:groupIndex1-1], groups[groupIndex1:]...)
				}
			}
		}
	}
	for i := 0; i < len(groups); i++ {
		sort.Strings(groups[i])
	}

	str := ""
	for i := 0; i < len(baseStr); i++ {
		c := baseStr[i : i+1]
		groupIndex := -1
		for j := 0; j < len(groups); j++ {
			for k := 0; k < len(groups[j]); k++ {
				if groups[j][k] == c {
					groupIndex = j
					break
				}
			}
			if groupIndex != -1 {
				break
			}
		}
		if groupIndex == -1 {
			str += c
		} else {
			str += groups[groupIndex][0]
		}
	}

	return str
}

func main() {
	fmt.Println(smallestEquivalentString("acb", "bdc", "edc"))
	fmt.Println(smallestEquivalentString("parker", "morris", "parser"))
	fmt.Println(smallestEquivalentString("hello", "world", "hold"))
	fmt.Println(smallestEquivalentString("leetcode", "programs", "sourcecode"))
	fmt.Println(smallestEquivalentString("bcfeaabddgcdaefcbfadggfagfgfedeefbebdbeefbecggcgge", "feegaacabcfadggfcaabcbadbbecbfdcabgeaegfcagdfggdgg", "mytnpodxbwxcxcplapgrqjzkfrkizffkbquwqbkxmpqjmxykvb"))
}
