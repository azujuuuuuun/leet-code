function findSpecialInteger(arr: number[]): number {
  const countMap = new Map<number, number>();
  for (const elm of arr) {
    const count = countMap.get(elm);
    countMap.set(elm, count ? count + 1 : 1);
  }
  for (const [elm, count] of countMap.entries()) {
    if (count > arr.length / 4) {
      return elm;
    }
  }
  return -1;
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
