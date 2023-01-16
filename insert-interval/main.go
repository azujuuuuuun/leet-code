package main

import (
	"fmt"
	"math"
)

func addNewInterval(intervals [][]int, newInterval []int) [][]int {
	appended := [][]int{}
	inserted := false
	for i := 0; i < len(intervals); i++ {
		interval := intervals[i]

		if inserted {
			appended = append(appended, interval)
			continue
		}

		if i == len(intervals)-1 {
			if newInterval[0] <= interval[0] {
				appended = append(appended, newInterval)
				appended = append(appended, interval)
			} else {
				appended = append(appended, interval)
				appended = append(appended, newInterval)
			}
		} else {
			if newInterval[0] <= interval[0] {
				appended = append(appended, newInterval)
				appended = append(appended, interval)
			} else {
				appended = append(appended, interval)
				continue
			}
		}
		inserted = true
	}
	return appended
}

func mergeIntervals(intervals [][]int) [][]int {
	merged := [][]int{}

	start := intervals[0][0]
	end := intervals[0][1]
	for i := 1; i < len(intervals); i++ {
		interval := intervals[i]
		si := interval[0]
		ei := interval[1]
		if start == si {
			end = int(math.Max(float64(end), float64(ei)))
		} else if start < si && si <= end {
			end = int(math.Max(float64(end), float64(ei)))
		} else {
			merged = append(merged, []int{start, end})
			start = si
			end = ei
		}
		if i == len(intervals)-1 {
			merged = append(merged, []int{start, end})
		}
	}

	return merged
}

func insert(intervals [][]int, newInterval []int) [][]int {
	if len(intervals) == 0 {
		return [][]int{newInterval}
	}
	added := addNewInterval(intervals, newInterval)
	return mergeIntervals(added)
}

func main() {
	fmt.Println(insert([][]int{{1, 3}, {6, 9}}, []int{1, 5}), [][]int{{1, 5}, {6, 9}})
	fmt.Println(insert([][]int{{1, 3}, {6, 9}}, []int{2, 5}), [][]int{{1, 5}, {6, 9}})
	fmt.Println(insert([][]int{{1, 2}, {3, 5}, {6, 7}, {8, 10}, {12, 16}}, []int{4, 8}), [][]int{{1, 2}, {3, 10}, {12, 16}})
}
