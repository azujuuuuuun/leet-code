package main

import (
	"fmt"
	"strings"
)

func findWords(words []string) []string {
	rows := []string{"qwertyuiop", "asdfghjkl", "zxcvbnm"}

	ansWords := []string{}
	for i := 0; i < len(words); i++ {
		rowIndex := 0
		for j := 0; j < len(rows); j++ {
			if strings.Contains(rows[j], strings.ToLower(string(words[i][0]))) {
				rowIndex = j
			}
		}
		sameRow := true
		for j := 1; j < len(words[i]); j++ {
			if !strings.Contains(rows[rowIndex], strings.ToLower(string(words[i][j]))) {
				sameRow = false
				break
			}
		}
		if sameRow {
			ansWords = append(ansWords, words[i])
		}
	}

	return ansWords
}

func main() {
	fmt.Println(findWords([]string{"Hello", "Alaska", "Dad", "Peace"}))
	fmt.Println(findWords([]string{"omk"}))
	fmt.Println(findWords([]string{"adsdf", "sfd"}))
}
