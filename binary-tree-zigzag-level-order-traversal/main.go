package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type QNode struct {
	Depth int
	*TreeNode
}

func zigzagLevelOrder(root *TreeNode) [][]int {
	if root == nil {
		return [][]int{}
	}

	queue := []*QNode{{1, root}}
	levelNodes := [][]int{}

	for len(queue) != 0 {
		node := queue[0]
		queue = queue[1:]

		if len(levelNodes) < node.Depth {
			levelNodes = append(levelNodes, []int{node.Val})
		} else {
			levelNodes[node.Depth-1] = append(levelNodes[node.Depth-1], node.Val)
		}

		if node.Left != nil {
			queue = append(queue, &QNode{node.Depth + 1, node.Left})
		}
		if node.Right != nil {
			queue = append(queue, &QNode{node.Depth + 1, node.Right})
		}
	}

	for i := 0; i < len(levelNodes); i++ {
		if i%2 == 1 {
			for j, k := 0, len(levelNodes[i])-1; j < k; j, k = j+1, k-1 {
				levelNodes[i][j], levelNodes[i][k] = levelNodes[i][k], levelNodes[i][j]
			}
		}
	}

	return levelNodes
}
