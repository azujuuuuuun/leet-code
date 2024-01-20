function sumSubarrayMins(arr: number[]): number {
  const stack: number[] = [];
  const modulo = 1000000007;
  let sum = 0;
  for (let i = 0; i <= arr.length; i++) {
    while (
      stack.length !== 0 &&
      (i === arr.length || arr[stack[stack.length - 1]] >= arr[i])
    ) {
      const mid = stack[stack.length - 1];
      stack.pop();
      const left = stack.length === 0 ? -1 : stack[stack.length - 1];
      const right = i;
      const count = ((mid - left) * (right - mid)) % modulo;
      sum += (count * arr[mid]) % modulo;
      sum %= modulo;
    }
    stack.push(i);
  }
  return sum;
}

console.log(sumSubarrayMins([3, 1, 2, 4]), 17);
console.log(sumSubarrayMins([11, 81, 94, 43, 3]), 444);
