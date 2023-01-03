package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	if list1 == nil && list2 == nil {
		return nil
	}

	n1 := list1
	n2 := list2
	head := &ListNode{}
	n := head

	for {
		if n1 != nil && n2 == nil {
			n.Val = n1.Val
			n1 = n1.Next
		}

		if n1 == nil && n2 != nil {
			n.Val = n2.Val
			n2 = n2.Next
		}

		if n1 != nil && n2 != nil {
			if n1.Val <= n2.Val {
				n.Val = n1.Val
				n1 = n1.Next
			} else {
				n.Val = n2.Val
				n2 = n2.Next
			}
		}

		if n1 == nil && n2 == nil {
			n.Next = nil
			break
		}

		next := &ListNode{}
		n.Next = next
		n = n.Next
	}

	return head
}
