package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if (p == nil && q != nil) || (p != nil && q == nil) {
		return false
	}

	pq := []*TreeNode{p}
	qq := []*TreeNode{q}

	isSame := false
	for {
		pNode := pq[0]
		pq = pq[1:]
		qNode := qq[0]
		qq = qq[1:]

		if pNode.Val != qNode.Val {
			break
		}

		if pNode.Left == nil && pNode.Right == nil && qNode.Left == nil && qNode.Right == nil {
			if len(pq) == 0 && len(qq) == 0 {
				isSame = true
				break
			}
			continue
		}

		if (pNode.Left == nil && qNode.Left != nil) || (pNode.Left != nil && qNode.Left == nil) || (pNode.Right == nil && qNode.Right != nil) || (pNode.Right != nil && qNode.Right == nil) {
			break
		}

		if pNode.Left != nil && qNode.Left != nil {
			pq = append(pq, pNode.Left)
			qq = append(qq, qNode.Left)
		}
		if pNode.Right != nil && qNode.Right != nil {
			pq = append(pq, pNode.Right)
			qq = append(qq, qNode.Right)
		}
	}

	return isSame
}
