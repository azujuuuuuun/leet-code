package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func middleNode(head *ListNode) *ListNode {
	var values []*ListNode
	now := head
	for {
		values = append(values, now)
		if now.Next == nil {
			break
		}
		now = now.Next
	}

	return values[len(values)/2]
}
