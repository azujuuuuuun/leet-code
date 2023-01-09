package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type QNode struct {
	*TreeNode
	Depth int
}

func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return [][]int{}
	}

	values := [][]int{}

	queue := []*QNode{}
	queue = append(queue, &QNode{TreeNode: root, Depth: 1})

	for len(queue) != 0 {
		qNode := queue[0]
		queue = queue[1:]

		if len(values) < qNode.Depth {
			values = append(values, []int{qNode.Val})
		} else {
			values[qNode.Depth-1] = append(values[qNode.Depth-1], qNode.Val)
		}

		if qNode.Left != nil {
			queue = append(queue, &QNode{TreeNode: qNode.Left, Depth: qNode.Depth + 1})
		}
		if qNode.Right != nil {
			queue = append(queue, &QNode{TreeNode: qNode.Right, Depth: qNode.Depth + 1})
		}
	}

	return values
}
