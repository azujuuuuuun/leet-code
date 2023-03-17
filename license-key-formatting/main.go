package main

import (
	"fmt"
	"strings"
)

func licenseKeyFormatting(s string, k int) string {
	joined := strings.Join(strings.Split(s, "-"), "")

	groups := []string{}
	rest := len(joined) % k

	if rest != 0 {
		groups = append(groups, strings.ToUpper(joined[0:rest]))
		joined = joined[rest:]
	}

	for joined != "" {
		groups = append(groups, strings.ToUpper(joined[0:k]))
		joined = joined[k:]
	}

	return strings.Join(groups, "-")
}

func main() {
	fmt.Println(licenseKeyFormatting("5F3Z-2e-9-w", 4))
	fmt.Println(licenseKeyFormatting("2-5g-3-J", 2))
	fmt.Println(licenseKeyFormatting("5F3Z-2e-9-wa", 4))
}
