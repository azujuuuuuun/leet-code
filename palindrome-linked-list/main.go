package main

type ListNode struct {
	Val  int
	Next *ListNode
}

func isPalindrome(head *ListNode) bool {
	var values []int
	now := head
	for {
		values = append(values, now.Val)

		if now.Next == nil {
			break
		}

		now = now.Next
	}

	ans := true
	for i := 0; i < len(values); i++ {
		forward := values[i]
		backward := values[len(values)-1-i]
		if forward != backward {
			ans = false
		}
	}

	return ans
}
