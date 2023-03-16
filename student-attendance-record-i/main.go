package main

import (
	"fmt"
	"strings"
)

func checkRecord(s string) bool {
	if strings.Count(s, "A") >= 2 {
		return false
	}

	if strings.Contains(s, "LLL") {
		return false
	}

	return true
}

func main() {
	fmt.Println(checkRecord("PPALLP"))
	fmt.Println(checkRecord("PPALLL"))
}
