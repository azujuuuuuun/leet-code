package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func search(node *TreeNode, sum int, targetSum int) bool {
	if node.Left == nil && node.Right == nil {
		return sum+node.Val == targetSum
	}
	if node.Left != nil && node.Right == nil {
		return search(node.Left, sum+node.Val, targetSum)
	}
	if node.Left == nil && node.Right != nil {
		return search(node.Right, sum+node.Val, targetSum)
	}
	return search(node.Left, sum+node.Val, targetSum) || search(node.Right, sum+node.Val, targetSum)
}

func hasPathSum(root *TreeNode, targetSum int) bool {
	if root == nil {
		return false
	}
	return search(root, 0, targetSum)
}

func main() {
	fmt.Println(hasPathSum(&TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 11,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
		},
		Right: &TreeNode{
			Val: 8,
			Left: &TreeNode{
				Val: 13,
			},
			Right: &TreeNode{
				Val: 4,
				Right: &TreeNode{
					Val: 1,
				},
			},
		},
	}, 22))
	fmt.Println(hasPathSum(&TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
		Right: &TreeNode{
			Val: 3,
		},
	}, 5))
	fmt.Println(hasPathSum(nil, 0))
	fmt.Println(hasPathSum(&TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
		},
	}, 1))
}
