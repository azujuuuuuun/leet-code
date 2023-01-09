package main

import (
	"container/heap"
	"fmt"
)

type IntHeap [][]int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i][1] < h[j][1] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.([]int))
}

func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func topKFrequent(nums []int, k int) []int {
	count := map[int]int{}
	for _, n := range nums {
		count[n]++
	}

	h := &IntHeap{}
	heap.Init(h)

	for n, f := range count {
		heap.Push(h, []int{n, f})
		if h.Len() > k {
			heap.Pop(h)
		}
	}

	freqElems := []int{}
	for h.Len() > 0 {
		n := heap.Pop(h).([]int)[0]
		freqElems = append(freqElems, n)
	}

	return freqElems
}

func main() {
	fmt.Println(topKFrequent([]int{1, 1, 1, 2, 2, 3}, 2))
	fmt.Println(topKFrequent([]int{1}, 1))
}
