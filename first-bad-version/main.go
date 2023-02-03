package main

/**
 * Forward declaration of isBadVersion API.
 * @param   version   your guess about first bad version
 * @return 	 	      true if current version is bad
 *			          false if current version is good
 * func isBadVersion(version int) bool;
 */

func firstBadVersion(n int) int {
	left := 0
	right := n
	for right-left > 1 {
		middle := left + (right-left)/2
		if isBadVersion(middle) {
			right = middle
		} else {
			left = middle
		}
	}
	return right
}
