package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSymmetric(root *TreeNode) bool {
	if root.Left == nil && root.Right == nil {
		return true
	}
	if (root.Left != nil && root.Right == nil) || (root.Left == nil && root.Right != nil) {
		return false
	}

	lq := []*TreeNode{root.Left}
	rq := []*TreeNode{root.Right}

	symmetric := true
	for len(lq) != 0 && len(rq) != 0 {
		lNode := lq[0]
		lq = lq[1:]
		rNode := rq[0]
		rq = rq[1:]

		if lNode.Val != rNode.Val {
			symmetric = false
			break
		}

		if lNode.Left == nil && rNode.Right != nil {
			symmetric = false
			break
		}
		if lNode.Left != nil && rNode.Right == nil {
			symmetric = false
			break
		}
		if lNode.Left != nil && rNode.Right != nil {
			lq = append(lq, lNode.Left)
			rq = append(rq, rNode.Right)
		}
		if lNode.Right == nil && rNode.Left != nil {
			symmetric = false
			break
		}
		if lNode.Right != nil && rNode.Left == nil {
			symmetric = false
			break
		}
		if lNode.Right != nil && rNode.Left != nil {
			lq = append(lq, lNode.Right)
			rq = append(rq, rNode.Left)
		}
	}

	return symmetric
}

func main() {
	fmt.Println(isSymmetric(&TreeNode{
		Val: 9,
		Left: &TreeNode{
			Val: -42,
			Right: &TreeNode{
				Val: 76,
				Right: &TreeNode{
					Val: 13,
				},
			},
		},
		Right: &TreeNode{
			Val: -42,
			Left: &TreeNode{
				Val: -42,
				Right: &TreeNode{
					Val: 13,
				},
			},
		},
	}))
}
