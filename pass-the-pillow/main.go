package main

import "fmt"

func passThePillow(n int, time int) int {
	if time/(n-1)%2 == 0 {
		// 1 -> n
		return 1 + time - (time / (n - 1) * (n - 1))
	} else {
		// n -> 1
		return n - (time - (time / (n - 1) * (n - 1)))
	}
}

func main() {
	fmt.Println(passThePillow(4, 5), 2)  // (5 / 3) % 2 == 1, 4 - 2 (5 - 3) = 2
	fmt.Println(passThePillow(4, 6), 1)  // (6 / 3) % 2 == 0, 1 + 0 (6 - 6) = 0
	fmt.Println(passThePillow(4, 7), 2)  // (7 / 3) % 2 == 0, 1 + 1 (7 - 6) = 2
	fmt.Println(passThePillow(4, 8), 3)  // (8 / 3) % 2 == 0, 1 + 2 (8 - 6) = 3
	fmt.Println(passThePillow(4, 10), 3) // (10 / 3) % 2 == 1, 4 - 1 (10 - 9) = 3
	fmt.Println(passThePillow(3, 2), 3)
}
