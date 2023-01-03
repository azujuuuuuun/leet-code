package main

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	var values []int
	n := x
	for {
		if n <= 0 {
			break
		}
		values = append(values, n%10)
		n = n / 10
	}

	var reversedValues []int
	for i := len(values) - 1; i >= 0; i-- {
		reversedValues = append(reversedValues, values[i])
	}

	ans := true
	for i := 0; i < len(values); i++ {
		if values[i] != reversedValues[i] {
			ans = false
			break
		}
	}

	return ans
}
