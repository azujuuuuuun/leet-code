package main

import (
	"fmt"
	"math"
)

type QueueNode struct {
	n    int
	cost int
}

func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	if len(flights) == 0 {
		return -1
	}

	flightsMap := map[int][][]int{}
	for i := 0; i < len(flights); i++ {
		from := flights[i][0]
		to := flights[i][1]
		price := flights[i][2]
		_, ok := flightsMap[from]
		if ok {
			flightsMap[from] = append(flightsMap[from], []int{to, price})
		} else {
			flightsMap[from] = [][]int{{to, price}}
		}
	}

	costs := map[int]int{}
	for i := 0; i < n; i++ {
		costs[i] = math.MaxInt
	}

	steps := 0
	queue := []QueueNode{{n: src, cost: 0}}
	for steps <= k && len(queue) != 0 {
		qLen := len(queue)

		for qLen != 0 {
			qLen--
			node := queue[0]
			queue = queue[1:]

			flight := flightsMap[node.n]
			for i := 0; i < len(flight); i++ {
				if node.cost+flight[i][1] > costs[flight[i][0]] {
					continue
				}
				costs[flight[i][0]] = node.cost + flight[i][1]
				queue = append(queue, QueueNode{flight[i][0], node.cost + flight[i][1]})
			}
		}

		steps++
	}

	if costs[dst] == math.MaxInt {
		return -1
	} else {
		return costs[dst]
	}
}

func main() {
	fmt.Println(findCheapestPrice(4, [][]int{{0, 1, 100}, {1, 2, 100}, {2, 0, 100}, {1, 3, 600}, {2, 3, 200}}, 0, 3, 1))
	fmt.Println(findCheapestPrice(3, [][]int{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}}, 0, 2, 1))
	fmt.Println(findCheapestPrice(3, [][]int{{0, 1, 100}, {1, 2, 100}, {0, 2, 500}}, 0, 2, 0))
	fmt.Println(findCheapestPrice(5, [][]int{{0, 1, 5}, {1, 2, 5}, {0, 3, 2}, {3, 1, 2}, {1, 4, 1}, {4, 2, 1}}, 0, 2, 2))
}
