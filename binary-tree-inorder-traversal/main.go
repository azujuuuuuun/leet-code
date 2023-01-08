package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func traverse(node *TreeNode, visited []int) []int {
	if node == nil {
		return visited
	}
	visited = traverse(node.Left, visited)
	visited = append(visited, node.Val)
	visited = traverse(node.Right, visited)
	return visited
}

func inorderTraversal(root *TreeNode) []int {
	return traverse(root, []int{})
}
