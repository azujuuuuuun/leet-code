function findSpecialInteger(arr: number[]): number {
  const countMap = new Map<number, number>();
  let maxCount = 1;
  let ans = arr[0];
  countMap.set(ans, 1);
  for (let i = 1; i < arr.length; i++) {
    const elm = arr[i];
    const count = countMap.get(elm);
    if (count) {
      countMap.set(elm, count + 1);
      if (count + 1 > maxCount) {
        maxCount = count + 1;
        ans = elm;
      }
    } else {
      countMap.set(elm, 1);
      if (1 > maxCount) {
        maxCount = 1;
        ans = elm;
      }
    }
  }
  return ans;
}

console.log(findSpecialInteger([1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
console.log(findSpecialInteger([1, 1]), 1);
console.log(findSpecialInteger([1, 2, 3, 3]), 3);
console.log(
  findSpecialInteger([
    1, 1, 1, 1, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 12, 12,
  ]),
  1
);
