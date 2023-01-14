package main

import "math"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getHeight(node *TreeNode, height int) int {
	if node == nil {
		return height
	}

	leftHeight := getHeight(node.Left, height+1)
	if leftHeight == math.MinInt {
		return math.MinInt
	}
	rightHeight := getHeight(node.Right, height+1)
	if rightHeight == math.MinInt {
		return math.MinInt
	}

	if math.Abs(float64(leftHeight)-float64(rightHeight)) <= 1 {
		return int(math.Max(float64(leftHeight), float64(rightHeight)))
	} else {
		return math.MinInt
	}
}

func isBalanced(root *TreeNode) bool {
	if root == nil {
		return true
	}

	leftHeight := getHeight(root.Left, 1)
	if leftHeight == math.MinInt {
		return false
	}
	rightHeight := getHeight(root.Right, 1)
	if rightHeight == math.MinInt {
		return false
	}

	if math.Abs(float64(leftHeight)-float64(rightHeight)) <= 1 {
		return isBalanced(root.Left) || isBalanced(root.Right)
	} else {
		return false
	}
}
