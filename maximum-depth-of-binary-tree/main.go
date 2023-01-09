package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type QueueNode struct {
	*TreeNode
	Depth int
}

func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	queue := []*QueueNode{}
	depth := 1
	queue = append(queue, &QueueNode{TreeNode: root, Depth: depth})

	for len(queue) != 0 {
		qNode := queue[len(queue)-1]
		queue = queue[:len(queue)-1]

		if qNode.Depth >= depth {
			depth = qNode.Depth
		}

		if qNode.Left != nil {
			queue = append(queue, &QueueNode{TreeNode: qNode.Left, Depth: qNode.Depth + 1})
		}
		if qNode.Right != nil {
			queue = append(queue, &QueueNode{TreeNode: qNode.Right, Depth: qNode.Depth + 1})
		}
	}

	return depth
}
