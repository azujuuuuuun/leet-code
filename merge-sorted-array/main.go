package main

func merge(nums1 []int, m int, nums2 []int, n int) {
	merged := []int{}
	i1 := 0
	i2 := 0
	for {
		if i1 == m && i2 == n {
			break
		}
		if i1 != m && i2 == n {
			merged = append(merged, nums1[i1])
			i1++
			continue
		}
		if i1 == m && i2 != n {
			merged = append(merged, nums2[i2])
			i2++
			continue
		}
		if nums1[i1] < nums2[i2] {
			merged = append(merged, nums1[i1])
			i1++
		} else if nums1[i1] == nums2[i2] {
			merged = append(merged, nums1[i1])
			i1++
		} else if nums1[i1] > nums2[i2] {
			merged = append(merged, nums2[i2])
			i2++
		}
	}

	for i, v := range merged {
		nums1[i] = v
	}
}
