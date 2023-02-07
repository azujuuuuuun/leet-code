package main

import "fmt"

func totalFruit(fruits []int) int {
	if len(fruits) <= 2 {
		return len(fruits)
	}

	fn := -1
	sn := -1
	sec := 0
	cur := 0
	total := 0
	for i := 0; i < len(fruits); i++ {
		if cur > total {
			total = cur
		}
		if fn == -1 {
			fn = fruits[i]
			sec++
			cur++
			if i == len(fruits)-1 {
				if cur > total {
					total = cur
				}
			}
			continue
		}
		if sn == -1 && fruits[i] == fn {
			sec++
			cur++
			if i == len(fruits)-1 {
				if cur > total {
					total = cur
				}
			}
			continue
		}
		if sn == -1 && fruits[i] != fn {
			sn = fruits[i]
			cur++
			sec = 1
			if i == len(fruits)-1 {
				if cur > total {
					total = cur
				}
			}
			continue
		}
		if fruits[i] == fn {
			cur++
			if i == len(fruits)-1 {
				if cur > total {
					total = cur
				}
			}
			fn, sn, sec = sn, fn, 1
			continue
		}
		if fruits[i] == sn {
			cur++
			sec++
			if i == len(fruits)-1 {
				if cur > total {
					total = cur
				}
			}
			continue
		}
		if cur > total {
			total = cur
		}
		fn, sn, cur, sec = sn, fruits[i], sec+1, 1
	}

	return total
}

func main() {
	fmt.Println(totalFruit([]int{1, 2, 1}))
	fmt.Println(totalFruit([]int{0, 1, 2, 2}))
	fmt.Println(totalFruit([]int{1, 2, 3, 2, 2}))
	fmt.Println(totalFruit([]int{1, 0, 1, 4, 1, 4, 1, 2, 3}))
	fmt.Println(totalFruit([]int{0, 1, 6, 6, 4, 4, 6}))
}
