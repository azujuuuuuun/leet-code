package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	var prev *ListNode = nil
	var last *ListNode = nil
	node := head

	for {
		if node.Next == nil {
			if prev != nil {
				node.Next = prev
			}
			last = node
			break
		}
		next := node.Next
		node.Next = prev
		prev = node
		node = next
	}

	return last
}
