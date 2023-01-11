package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func rec(node *ListNode, prev *ListNode) *ListNode {
	next := node.Next
	node.Next = prev

	if next == nil {
		return node
	}

	return rec(next, node)
}

func reverseList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	return rec(head, nil)
}
