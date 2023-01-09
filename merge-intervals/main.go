package main

import (
	"fmt"
	"sort"
)

type ByStart [][]int

func (s ByStart) Len() int           { return len(s) }
func (s ByStart) Less(i, j int) bool { return s[i][0] < s[j][0] }
func (s ByStart) Swap(i, j int)      { s[i], s[j] = s[j], s[i] }

func merge(intervals [][]int) [][]int {
	sort.Sort(ByStart(intervals))

	merged := [][]int{}
	start := intervals[0][0]
	end := intervals[0][1]
	for i := 0; i < len(intervals); i++ {
		if intervals[i][0] > end {
			merged = append(merged, []int{start, end})
			start = intervals[i][0]
			end = intervals[i][1]
		} else {
			if intervals[i][1] > end {
				end = intervals[i][1]
			}
		}
		if i == len(intervals)-1 {
			merged = append(merged, []int{start, end})
		}
	}

	return merged
}

func main() {
	fmt.Println(merge([][]int{{1, 3}, {2, 6}, {8, 10}, {15, 18}}))
	fmt.Println(merge([][]int{{1, 4}, {4, 5}}))
	fmt.Println(merge([][]int{{1, 3}}))
	fmt.Println(merge([][]int{{1, 4}, {2, 3}}))
}
