package main

import "math"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minDiffInBST(root *TreeNode) int {
	vals := []int{}
	queue := []*TreeNode{root}

	for len(queue) != 0 {
		node := queue[0]
		queue = queue[1:]

		vals = append(vals, node.Val)
		if node.Left != nil {
			queue = append(queue, node.Left)
		}
		if node.Right != nil {
			queue = append(queue, node.Right)
		}
	}

	minDiff := math.MaxInt
	for i := 0; i < len(vals); i++ {
		for j := 0; j < len(vals); j++ {
			if i == j {
				continue
			}
			if vals[i] < vals[j] {
				if vals[j]-vals[i] < minDiff {
					minDiff = vals[j] - vals[i]
				}
			} else {
				if vals[i]-vals[j] < minDiff {
					minDiff = vals[i] - vals[j]
				}
			}
		}
	}

	return minDiff
}
