package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
	values := []int{}
	node := root
	stack := []*TreeNode{}

	for {
		if node == nil && len(stack) == 0 {
			break
		}

		if node == nil && len(stack) != 0 {
			node = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}

		values = append(values, node.Val)

		if node.Left == nil && node.Right == nil {
			node = nil
		} else if node.Left != nil && node.Right == nil {
			node = node.Left
		} else if node.Left == nil && node.Right != nil {
			node = node.Right
		} else if node.Left != nil && node.Right != nil {
			stack = append(stack, node.Right)
			node = node.Left
		}
	}

	return values
}
