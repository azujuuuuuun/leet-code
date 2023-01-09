package main

import (
	"reflect"
	"testing"
)

func TestLevelOrder(t *testing.T) {
	tests := []struct {
		name string
		root *TreeNode
		want [][]int
	}{{
		name: "Case 1",
		root: &TreeNode{
			Val: 3,
			Left: &TreeNode{
				Val: 9,
			},
			Right: &TreeNode{
				Val: 20,
				Left: &TreeNode{
					Val: 15,
				},
				Right: &TreeNode{
					Val: 7,
				},
			},
		},
		want: [][]int{{3}, {9, 20}, {15, 7}},
	}, {
		name: "Case 2",
		root: &TreeNode{
			Val: 1,
		},
		want: [][]int{{1}},
	}, {
		name: "Case 3",
		root: nil,
		want: [][]int{},
	}}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := levelOrder(tt.root)

			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("got = %v, want = %v", got, tt.want)
			}
		})
	}
}
