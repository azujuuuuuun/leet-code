type NumArray struct {
	total []int
}

func Constructor(nums []int) NumArray {
	total := []int{nums[0]}
	for i := 1; i < len(nums); i++ {
		total = append(total, total[i-1]+nums[i])
	}
	return NumArray{total}
}

func (this *NumArray) SumRange(left int, right int) int {
	if left == 0 {
		return this.total[right]
	}
	return this.total[right] - this.total[left-1]
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.SumRange(left,right);
 */