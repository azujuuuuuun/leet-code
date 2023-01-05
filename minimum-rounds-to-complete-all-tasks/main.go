package main

import "fmt"

func minimumRounds(tasks []int) int {
	levelCounter := map[int]int{}
	for i := 0; i < len(tasks); i++ {
		levelCounter[tasks[i]]++
	}
	rounds := 0
	for _, v := range levelCounter {
		if v == 1 {
			return -1
		}
		if v%3 == 0 {
			rounds += v / 3
		} else if v%3 == 1 {
			rounds += v/3 + 1
		} else if v%3 == 2 {
			rounds += v/3 + 1
		}
	}
	return rounds
}

func main() {
	fmt.Println(minimumRounds([]int{2, 2, 3, 3, 2, 4, 4, 4, 4, 4}))
	fmt.Println(minimumRounds([]int{2, 3, 3}))
	fmt.Println(minimumRounds([]int{5, 5, 5, 5}))
}
