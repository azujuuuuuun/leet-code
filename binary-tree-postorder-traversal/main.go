package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func search(node *TreeNode, values *[]int) {
	if node.Left != nil {
		search(node.Left, values)
	}
	if node.Right != nil {
		search(node.Right, values)
	}
	*values = append(*values, node.Val)
}

func postorderTraversal(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}

	values := []int{}

	search(root, &values)

	return values
}

func main() {
	fmt.Println(postorderTraversal(&TreeNode{
		Val: 1,
		Right: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 3,
			},
		},
	}))
	fmt.Println(postorderTraversal(nil))
	fmt.Println(postorderTraversal(&TreeNode{
		Val: 1,
	}))
}
