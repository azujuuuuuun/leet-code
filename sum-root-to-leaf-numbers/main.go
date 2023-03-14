package main

import (
	"fmt"
	"math"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func dfs(node *TreeNode, path []int, sum *int) {
	path = append(path, node.Val)

	if node.Left == nil && node.Right == nil {
		for i := 0; i < len(path); i++ {
			*sum += path[len(path)-1-i] * int(math.Pow(10, float64(i)))
		}
		return
	}

	if node.Left != nil {
		dfs(node.Left, path, sum)
	}

	if node.Right != nil {
		dfs(node.Right, path, sum)
	}
}

func sumNumbers(root *TreeNode) int {
	if root == nil {
		return 0
	}

	sum := 0

	dfs(root, []int{}, &sum)

	return sum
}

func main() {
	fmt.Println(sumNumbers(&TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}))

	fmt.Println(sumNumbers(&TreeNode{
		Val: 4,
		Left: &TreeNode{
			Val: 9,
			Left: &TreeNode{
				Val: 5,
			},
			Right: &TreeNode{
				Val: 1,
			},
		},
		Right: &TreeNode{
			Val: 0,
		},
	}))
}
