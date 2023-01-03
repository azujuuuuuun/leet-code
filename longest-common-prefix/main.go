package main

import "strings"

func longestCommonPrefix(strs []string) string {
	str := strs[0]
	prefix := ""
	for i := len(str); i >= 0; i-- {
		tmpPrefix := str[0:i]
		include := true
		for j := 1; j < len(strs); j++ {
			if !strings.HasPrefix(strs[j], tmpPrefix) {
				include = false
				break
			}
		}
		if include {
			prefix = tmpPrefix
			break
		}
	}
	return prefix
}
