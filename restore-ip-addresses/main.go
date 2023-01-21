package main

import (
	"fmt"
	"strconv"
	"strings"
)

func search(n string, nums []string, s string, set *map[string]bool) {
	if len(n) > 1 && n[0:1] == "0" {
		return
	}
	nInt, _ := strconv.Atoi(n)
	if nInt > 255 {
		return
	}
	nums = append(nums, n)
	if len(nums) == 4 && len(s) == 0 {
		(*set)[strings.Join(nums, ".")] = true
	}
	if len(s) == 0 {
		return
	}
	search(s[0:1], nums, s[1:], set)
	if len(s) >= 2 {
		search(s[0:2], nums, s[2:], set)
	}
	if len(s) >= 3 {
		search(s[0:3], nums, s[3:], set)
	}
}

func restoreIpAddresses(s string) []string {
	if len(s) < 4 {
		return []string{}
	}

	set := map[string]bool{}
	search(s[0:1], []string{}, s[1:], &set)
	search(s[0:2], []string{}, s[2:], &set)
	search(s[0:3], []string{}, s[3:], &set)

	ipAddresses := []string{}
	for k := range set {
		ipAddresses = append(ipAddresses, k)
	}

	return ipAddresses
}

func main() {
	fmt.Println(restoreIpAddresses("25525511135"))
	fmt.Println(restoreIpAddresses("0000"))
	fmt.Println(restoreIpAddresses("101023"))
}
