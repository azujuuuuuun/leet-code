package main

import "fmt"

func dfs(node int, parent int, adj [][]int, hasApple []bool) int {
	totalTime, childTime := 0, 0

	for _, child := range adj[node] {
		if child == parent {
			continue
		}

		childTime = dfs(child, node, adj, hasApple)
		if childTime > 0 || hasApple[child] {
			totalTime += childTime + 2
		}
	}

	return totalTime
}

func minTime(n int, edges [][]int, hasApple []bool) int {
	adj := [][]int{}
	for i := 0; i < n; i++ {
		adj = append(adj, []int{})
	}
	for _, e := range edges {
		adj[e[0]] = append(adj[e[0]], e[1])
		adj[e[1]] = append(adj[e[1]], e[0])
	}
	return dfs(0, -1, adj, hasApple)
}

func main() {
	fmt.Println(minTime(7, [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, []bool{false, false, true, false, true, true, false}))
	fmt.Println(minTime(7, [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, []bool{false, false, true, false, false, true, false}))
	fmt.Println(minTime(7, [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, []bool{false, false, false, false, false, false, false}))
}
