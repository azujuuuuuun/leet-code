function lengthOfLIS(nums: number[]): number {
  const memo: number[] = [];
  for (let i = 0; i < nums.length; i++) {
    memo.push(1);
  }
  for (let i = nums.length - 1; i >= 0; i--) {
    let max = 1;
    for (let j = i + 1; j < nums.length; j++) {
      const num = nums[i] < nums[j] ? memo[j] + 1 : 1;
      if (num > max) {
        max = num;
      }
    }
    memo[i] = max;
  }
  let max = 0;
  for (let i = 0; i < memo.length; i++) {
    if (memo[i] > max) {
      max = memo[i];
    }
  }
  return max;
}

console.log(lengthOfLIS([10, 9, 2, 5, 3, 7, 101, 18]), 4);
console.log(lengthOfLIS([0, 1, 0, 3, 2, 3]), 4);
console.log(lengthOfLIS([7, 7, 7, 7, 7, 7, 7]), 1);
console.log(lengthOfLIS([1, 2, 3, 4, 5]), 5);
