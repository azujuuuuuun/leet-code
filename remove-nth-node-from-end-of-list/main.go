package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	sz := 0
	node := head
	for node != nil {
		sz++
		node = node.Next
	}

	if sz == n {
		return head.Next
	}

	count := 1
	node = head
	for {
		if n == 1 {
			if count == sz-1 {
				node.Next = nil
				break
			}
		} else {
			if count == sz-n {
				node.Next = node.Next.Next
				break
			}
		}

		node = node.Next
		count++
	}

	return head
}
