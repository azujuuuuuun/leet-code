package main

import (
	"fmt"
	"sort"
)

type ByFreq [][]int

func (f ByFreq) Len() int {
	return len(f)
}

func (f ByFreq) Swap(i, j int) {
	f[i], f[j] = f[j], f[i]
}

func (f ByFreq) Less(i, j int) bool {
	return f[i][1] < f[j][1]
}

func topKFrequent(nums []int, k int) []int {
	count := map[int]int{}
	for _, n := range nums {
		count[n]++
	}

	freq := [][]int{}
	for k, v := range count {
		freq = append(freq, []int{k, v})
	}
	sort.Sort(ByFreq(freq))

	freqElems := []int{}
	for i := 0; i < k; i++ {
		freqElems = append(freqElems, freq[len(freq)-1-i][0])
	}

	return freqElems
}

func main() {
	fmt.Println(topKFrequent([]int{1, 1, 1, 2, 2, 3}, 2))
	fmt.Println(topKFrequent([]int{1}, 1))
}
