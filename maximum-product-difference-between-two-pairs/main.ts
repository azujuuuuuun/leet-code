function maxProductDifference(nums: number[]): number {
  let max = 0;
  let max2 = 0;
  let min = 10001;
  let min2 = 10001;
  for (const num of nums) {
    if (num >= max) {
      max2 = max;
      max = num;
    } else if (num >= max2) {
      max2 = num;
    }
    if (num <= min) {
      min2 = min;
      min = num;
    } else if (num <= min2) {
      min2 = num;
    }
  }
  return max * max2 - min * min2;
}

console.log(maxProductDifference([5, 6, 2, 7, 4]), 34);
console.log(maxProductDifference([4, 2, 5, 9, 7, 4, 8]), 64);
