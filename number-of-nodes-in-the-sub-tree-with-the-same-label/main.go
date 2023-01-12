package main

import "fmt"

func countLabel(node int, parent int, adj [][]int, labels string, ansCounter *map[int]int) map[string]int {
	labelCounter := map[string]int{}
	label := labels[node : node+1]
	labelCounter[label]++

	for i := 0; i < len(adj[node]); i++ {
		if adj[node][i] == parent {
			continue
		}

		lc := countLabel(adj[node][i], node, adj, labels, ansCounter)
		for k, v := range lc {
			labelCounter[k] += v
		}
		// for k, v := range ac {
		// 	ansCounter[k] = v
		// }
	}

	c := labelCounter[label]
	(*ansCounter)[node] = c

	return labelCounter
}

func countSubTrees(n int, edges [][]int, labels string) []int {
	adj := [][]int{}
	for i := 0; i < n; i++ {
		adj = append(adj, []int{})
	}
	for i := 0; i < len(edges); i++ {
		adj[edges[i][0]] = append(adj[edges[i][0]], edges[i][1])
		adj[edges[i][1]] = append(adj[edges[i][1]], edges[i][0])
	}

	ansCounter := map[int]int{}
	countLabel(0, -1, adj, labels, &ansCounter)

	ans := []int{}
	for i := 0; i < n; i++ {
		ans = append(ans, 0)
	}
	for k, v := range ansCounter {
		ans[k] = v
	}

	return ans
}

func main() {
	fmt.Println(countSubTrees(7, [][]int{{0, 1}, {0, 2}, {1, 4}, {1, 5}, {2, 3}, {2, 6}}, "abaedcd"))
	fmt.Println(countSubTrees(4, [][]int{{0, 1}, {1, 2}, {0, 3}}, "bbbb"))
	fmt.Println(countSubTrees(5, [][]int{{0, 1}, {0, 2}, {1, 3}, {0, 4}}, "aabab"))
	fmt.Println(countSubTrees(4, [][]int{{0, 2}, {0, 3}, {1, 2}}, "aeed"))
}
