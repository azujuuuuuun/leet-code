package main

import (
	"fmt"
	"strconv"
	"strings"
)

// func dfs(index int, n int, nums []int, path []int, ans *[][]int, ansMap *map[string]bool) {
// func dfs(index int, n int, nums []int, path []int, ansMap *map[string]bool) {
// 	path = append(path, n)
// 	if len(path) >= 2 {
// 		// *ans = append(*ans, path)
// 		pathS := []string{}
// 		for j := 0; j < len(path); j++ {
// 			pathS = append(pathS, strconv.Itoa(path[j]))
// 		}
// 		(*ansMap)[strings.Join(pathS, ",")] = true
// 	}
// 	for i := index + 1; i < len(nums); i++ {
// 		next := nums[i]
// 		if next < n {
// 			continue
// 		}
// 		// dfs(i, next, nums, path, ans, ansMap)
// 		dfs(i, next, nums, path, ansMap)
// 	}
// }

// func findSubsequences(nums []int) [][]int {
// 	if len(nums) == 1 {
// 		return [][]int{}
// 	}
// 	// ans := [][]int{}
// 	ansMap := map[string]bool{}
//
// 	for i, n := range nums {
// 		// dfs(i, n, nums, []int{}, &ans, &ansMap)
// 		dfs(i, n, nums, []int{}, &ansMap)
// 	}
//
// 	ans := [][]int{}
// 	for k := range ansMap {
// 		strs := strings.Split(k, ",")
// 		path := []int{}
// 		for i := 0; i < len(strs); i++ {
// 			n, _ := strconv.Atoi(strs[i])
// 			path = append(path, n)
// 		}
// 		ans = append(ans, path)
// 	}
//
// 	return ans
// }

func dfs(index int, n int, nums []int, path []int, ansMap *map[string]bool) {
	path = append(path, n)
	if len(path) >= 2 {
		pathStrs := []string{}
		for i := 0; i < len(path); i++ {
			pathStrs = append(pathStrs, strconv.Itoa(path[i]))
		}
		(*ansMap)[strings.Join(pathStrs, ",")] = true
	}
	for i := index + 1; i < len(nums); i++ {
		next := nums[i]
		if next < n {
			continue
		}
		dfs(i, next, nums, path, ansMap)
	}
}

func findSubsequences(nums []int) [][]int {
	if len(nums) == 1 {
		return [][]int{}
	}

	ansMap := map[string]bool{}
	for i, n := range nums {
		dfs(i, n, nums, []int{}, &ansMap)
	}

	ans := [][]int{}
	for k := range ansMap {
		strs := strings.Split(k, ",")
		path := []int{}
		for i := 0; i < len(strs); i++ {
			n, _ := strconv.Atoi(strs[i])
			path = append(path, n)
		}
		ans = append(ans, path)
	}

	return ans
}

func main() {
	fmt.Println(findSubsequences([]int{4, 6, 7, 7}))
	fmt.Println(findSubsequences([]int{4, 4, 3, 2, 1}))
	fmt.Println(findSubsequences([]int{5, 5, 3, 4, 5}))
}
