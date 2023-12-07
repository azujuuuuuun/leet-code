function largestOddNumber(num: string): string {
  for (let i = 0; i < num.length; i++) {
    if (Number(num[num.length - 1 - i]) % 2 === 1) {
      return num.substring(0, num.length - i);
    }
  }
  return "";
}

console.log(largestOddNumber("52"), "5");
console.log(largestOddNumber("4206"), "");
console.log(largestOddNumber("35427"), "35427");
