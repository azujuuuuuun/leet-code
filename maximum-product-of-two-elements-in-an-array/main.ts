function maxProduct(nums: number[]): number {
  let first = 0;
  let second = 0;
  for (const num of nums) {
    if (num >= first) {
      second = first;
      first = num;
    } else if (num >= second) {
      second = num;
    }
  }
  return (first - 1) * (second - 1);
}

console.log(maxProduct([3, 4, 5, 2]), 12);
console.log(maxProduct([1, 5, 4, 5]), 16);
console.log(maxProduct([3, 7]), 12);
