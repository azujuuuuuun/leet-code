package main

import (
	"fmt"
	"strings"
)

func countAsterisks(s string) int {
	pairs := strings.Split(s, "|")
	count := 0

	for i := 0; i < len(pairs); i++ {
		if i%2 == 1 {
			continue
		}

		count += strings.Count(pairs[i], "*")
	}

	return count
}

func main() {
	fmt.Println(countAsterisks("l|*e*et|c**o|*de|"))
	fmt.Println(countAsterisks("iamprogrammer"))
	fmt.Println(countAsterisks("yo|uar|e**|b|e***au|tifu|l"))
	fmt.Println(countAsterisks("iamprogrammer*"))
}
