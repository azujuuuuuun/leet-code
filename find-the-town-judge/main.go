package main

import "fmt"

func findJudge(n int, trust [][]int) int {
	if len(trust) == 0 {
		if n == 1 {
			return 1
		} else {
			return -1
		}
	}

	cnt := map[int][]int{}
	for i := 0; i < len(trust); i++ {
		_, ok := cnt[trust[i][0]]
		if ok {
			cnt[trust[i][0]] = append(cnt[trust[i][0]], trust[i][1])
		} else {
			cnt[trust[i][0]] = []int{trust[i][1]}
		}
	}

	trusted := -1
	for i := 1; i <= n; i++ {
		_, ok := cnt[i]
		if !ok {
			trusted = i
		}
	}

	if trusted == -1 {
		return -1
	}

	for k, v := range cnt {
		if k == trusted {
			continue
		}
		includes := false
		for i := 0; i < len(v); i++ {
			if v[i] == trusted {
				includes = true
			}
		}
		if !includes {
			return -1
		}
	}

	return trusted
}

func main() {
	fmt.Println(findJudge(2, [][]int{{1, 2}}))
	fmt.Println(findJudge(3, [][]int{{1, 3}, {2, 3}}))
	fmt.Println(findJudge(3, [][]int{{1, 3}, {2, 3}, {3, 1}}))
	fmt.Println(findJudge(4, [][]int{{1, 3}, {1, 4}, {2, 3}, {2, 4}, {4, 3}}))
}
