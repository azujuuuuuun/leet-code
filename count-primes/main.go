package main

import "fmt"

func countPrimes(n int) int {
	if n <= 1 {
		return 0
	}

	isPrime := []bool{}
	for i := 0; i <= n; i++ {
		isPrime = append(isPrime, true)
	}
	isPrime[0] = false
	isPrime[1] = false

	for i := 2; i <= n; i++ {
		if !isPrime[i] {
			continue
		}

		for j := 2; i*j <= n; j++ {
			isPrime[i*j] = false
		}
	}

	count := 0
	for i := 0; i < n; i++ {
		if isPrime[i] {
			count++
		}
	}

	return count
}

func main() {
	fmt.Println(countPrimes(0))
	fmt.Println(countPrimes(1))
	fmt.Println(countPrimes(2))
	fmt.Println(countPrimes(3))
	fmt.Println(countPrimes(5))
}
