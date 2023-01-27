package main

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * func guess(num int) int;
 */

func guessNumber(n int) int {
	low := 1
	high := n

	for {
		middle := (low + high) / 2
		result := guess(middle)
		if result == 0 {
			return middle
		} else if result == -1 {
			high = middle - 1
		} else {
			low = middle + 1
		}
	}
}
