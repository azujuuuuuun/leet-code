function maxFrequencyElements(nums: number[]): number {
  const freq = {};
  let max = 0;
  for (const n of nums) {
    if (freq[n]) {
      freq[n]++;
    } else {
      freq[n] = 1;
    }
    if (freq[n] >= max) {
      max = freq[n];
    }
  }
  const count = Object.values(freq).filter((f) => f === max).length;
  return max * count;
}
