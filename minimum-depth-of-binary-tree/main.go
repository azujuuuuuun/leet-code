package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type QNode struct {
	*TreeNode
	depth int
}

func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	queue := []*QNode{}
	queue = append(queue, &QNode{root, 1})
	depth := 0
	for len(queue) != 0 {
		node := queue[0]
		queue = queue[1:]

		if node.Left == nil && node.Right == nil {
			depth = node.depth
			break
		}
		if node.Left != nil {
			queue = append(queue, &QNode{node.Left, node.depth + 1})
		}
		if node.Right != nil {
			queue = append(queue, &QNode{node.Right, node.depth + 1})
		}
	}

	return depth
}
