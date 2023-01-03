package main

import "strings"

func detectCapitalUse(word string) bool {
	upperWord := strings.ToUpper(word)
	lowerWord := strings.ToLower(word)
	if word == upperWord || word == lowerWord {
		return true
	}
	if word[0:1] == upperWord[0:1] && word[1:] == lowerWord[1:] {
		return true
	}
	return false
}
