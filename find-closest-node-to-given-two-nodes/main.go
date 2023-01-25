package main

import "fmt"

func traverse(node int, cost int, edges []int, distance *[]int) {
	if (*distance)[node] != -1 {
		return
	}
	(*distance)[node] = cost
	if edges[node] != -1 {
		traverse(edges[node], cost+1, edges, distance)
	}
}

func closestMeetingNode(edges []int, node1 int, node2 int) int {
	distance1 := []int{}
	distance2 := []int{}
	for i := 0; i < len(edges); i++ {
		if i == node1 {
			distance1 = append(distance1, 0)
		} else {
			distance1 = append(distance1, -1)
		}
		if i == node2 {
			distance2 = append(distance2, 0)
		} else {
			distance2 = append(distance2, -1)
		}
	}

	if edges[node1] != -1 {
		traverse(edges[node1], 1, edges, &distance1)
	}
	if edges[node2] != -1 {
		traverse(edges[node2], 1, edges, &distance2)
	}

	index := -1
	dist := 100000
	for i := 0; i < len(edges); i++ {
		dist1 := distance1[i]
		dist2 := distance2[i]

		if dist1 == -1 || dist2 == -1 {
			continue
		}

		if dist1 >= dist2 {
			if dist1 < dist {
				dist = dist1
				index = i
			}
		} else {
			if dist2 < dist {
				dist = dist2
				index = i
			}
		}
	}

	return index
}

func main() {
	fmt.Println(closestMeetingNode([]int{2, 2, 3, -1}, 0, 1))
	fmt.Println(closestMeetingNode([]int{1, 2, -1}, 0, 2))
	fmt.Println(closestMeetingNode([]int{51, -1, 75, 17, 71, -1, 52, 15, 58, 44, 16, 22, 47, 4, 60, 71, 32, 10, 84, -1, 51, 51, 17, -1, 15, 51, 32, 53, 83, 83, 47, -1, 67, -1, 47, 6, 46, 77, 9, -1, -1, 61, 11, 54, 6, 15, 7, 37, 8, 0, 9, 81, 30, 49, 38, -1, -1, 22, 68, 48, -1, 80, 36, 36, -1, 22, 52, 48, 82, 27, 68, 10, 56, 84, 32, 49, 75, 57, 77, 50, 36, 9, 61, 0, 49, 0, 16}, 27, 22))
	fmt.Println(closestMeetingNode([]int{1, 0, 0, 0, 0}, 2, 2))
}
