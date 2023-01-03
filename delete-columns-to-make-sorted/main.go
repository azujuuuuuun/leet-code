package main

import (
	"fmt"
	"sort"
	"strings"
)

func minDeletionSize(strs []string) int {
	var indices []int
	for i := 0; i < len(strs[0]); i++ {
		var column []string
		for j := 0; j < len(strs); j++ {
			column = append(column, strs[j][i:i+1])
		}
		sortedColumn := append([]string{}, column...)
		sort.Strings(sortedColumn)
		if strings.Join(column, "") != strings.Join(sortedColumn, "") {
			indices = append(indices, i)
		}
	}
	return len(indices)
}

func main() {
	fmt.Println(minDeletionSize([]string{"cba", "daf", "ghi"}))
	fmt.Println(minDeletionSize([]string{"a", "b"}))
	fmt.Println(minDeletionSize([]string{"zyx", "wvu", "tsr"}))
	fmt.Println(minDeletionSize([]string{"qowfc", "spyge", "sqbif", "vvrkk"}))
}
