package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func invert(node *TreeNode) {
	if node.Left == nil && node.Right == nil {
		return
	}
	node.Left, node.Right = node.Right, node.Left
	if node.Left != nil {
		invert(node.Left)
	}
	if node.Right != nil {
		invert(node.Right)
	}
}

func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	invert(root)
	return root
}
