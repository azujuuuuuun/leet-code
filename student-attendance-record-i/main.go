package main

import (
	"fmt"
)

func checkRecord(s string) bool {
	aCnt := 0

	for i, r := range s {
		if string(r) == "A" {
			aCnt++
		}

		if i+2 < len(s) && s[i:i+3] == "LLL" {
			return false
		}
	}

	if aCnt >= 2 {
		return false
	}

	return true
}

func main() {
	fmt.Println(checkRecord("PPALLP"))
	fmt.Println(checkRecord("PPALLL"))
}
