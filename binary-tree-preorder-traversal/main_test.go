package main

import (
	"reflect"
	"testing"
)

func TestPreorderTraversal(t *testing.T) {
	tests := []struct {
		name string
		root *TreeNode
		want []int
	}{{
		name: "Case 1",
		root: &TreeNode{
			Val: 1,
			Right: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 3,
				},
			},
		},
		want: []int{1, 2, 3},
	}, {
		name: "Case 2",
		root: nil,
		want: []int{},
	}, {
		name: "Case 3",
		root: &TreeNode{Val: 1},
		want: []int{1},
	}, {
		name: "Case 4",
		root: &TreeNode{
			Val: 1,
			Left: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 4,
				},
				Right: &TreeNode{
					Val: 5,
				},
			},
			Right: &TreeNode{
				Val: 3,
				Left: &TreeNode{
					Val: 6,
				},
				Right: &TreeNode{
					Val: 7,
				},
			},
		},
		want: []int{1, 2, 4, 5, 3, 6, 7},
	}}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := preorderTraversal(tt.root)

			if !reflect.DeepEqual(got, tt.want) {
				t.Errorf("got = %v, want = %v", got, tt.want)
			}
		})
	}
}
