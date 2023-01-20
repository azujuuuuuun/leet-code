package main

import (
	"strconv"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func traverse(node *TreeNode, path []string, paths *[]string) {
	path = append(path, strconv.Itoa(node.Val))
	if node.Left == nil && node.Right == nil {
		*paths = append(*paths, strings.Join(path, "->"))
		return
	}
	if node.Left != nil {
		traverse(node.Left, path, paths)
	}
	if node.Right != nil {
		traverse(node.Right, path, paths)
	}
}

func binaryTreePaths(root *TreeNode) []string {
	paths := []string{}

	traverse(root, []string{}, &paths)

	return paths
}
