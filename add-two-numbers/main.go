package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	var values1 []int
	n1 := l1
	for {
		values1 = append(values1, n1.Val)
		if n1.Next == nil {
			break
		}
		n1 = n1.Next
	}

	var values2 []int
	n2 := l2
	for {
		values2 = append(values2, n2.Val)
		if n2.Next == nil {
			break
		}
		n2 = n2.Next
	}

	var long []int
	var short []int
	if len(values1) >= len(values2) {
		long = values1
		short = values2
	} else {
		long = values2
		short = values1
	}

	for i := 0; i < len(long); i++ {
		shortN := 0
		if i < len(short) {
			shortN = short[i]
		}
		sum := long[i] + shortN
		val := (sum) % 10
		carry := sum / 10
		long[i] = val
		if carry > 0 {
			if i+1 >= len(long) {
				long = append(long, carry)
			} else {
				long[i+1] += carry
			}
		}
	}

	head := &ListNode{}
	n := head
	for i := 0; i < len(long); i++ {
		n.Val = long[i]
		if i == len(long)-1 {
			break
		}
		n.Next = &ListNode{}
		n = n.Next
	}

	return head
}
