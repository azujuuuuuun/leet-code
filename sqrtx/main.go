package main

func mySqrt(x int) int {
	ans := 0
	for i := 0; i*i <= x; i++ {
		ans = i
	}
	return ans
}
