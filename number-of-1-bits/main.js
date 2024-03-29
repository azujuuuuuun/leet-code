/**
 * @param {number} n - a positive integer
 * @return {number}
 */
var hammingWeight = function (n) {
  let count = 0;
  while (n !== 0) {
    count += n & 1;
    n >>>= 1;
  }
  return count;
};

console.log(hammingWeight(0b00000000000000000000000000001011), 3);
console.log(hammingWeight(0b00000000000000000000000010000000), 1);
console.log(hammingWeight(0b11111111111111111111111111111101), 31);
