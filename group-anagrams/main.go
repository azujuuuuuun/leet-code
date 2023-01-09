package main

import (
	"fmt"
	"sort"
	"strings"
)

func groupAnagrams(strs []string) [][]string {
	m := map[string][]string{}

	for _, s := range strs {
		chars := strings.Split(s, "")
		sort.Strings(chars)
		sortedS := strings.Join(chars, "")
		_, ok := m[sortedS]
		if ok {
			m[sortedS] = append(m[sortedS], s)
		} else {
			m[sortedS] = []string{s}
		}
	}

	groups := [][]string{}
	for _, g := range m {
		groups = append(groups, g)
	}

	return groups
}

func main() {
	fmt.Println(groupAnagrams([]string{"eat", "tea", "tan", "ate", "nat", "bat"}))
	fmt.Println(groupAnagrams([]string{""}))
	fmt.Println(groupAnagrams([]string{"a"}))
}
