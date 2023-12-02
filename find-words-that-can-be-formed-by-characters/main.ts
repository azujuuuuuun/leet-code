function countCharacters(words: string[], chars: string): number {
  const charCountMap = new Map<string, number>();
  for (const c of chars) {
    const charCount = charCountMap.get(c);
    if (charCount) {
      charCountMap.set(c, charCount + 1);
    } else {
      charCountMap.set(c, 1);
    }
  }

  let count = 0;
  for (const word of words) {
    const wordCharCountMap = new Map();
    for (const [key, value] of charCountMap.entries()) {
      wordCharCountMap.set(key, value);
    }
    let isGood = true;
    for (const c of word) {
      const charCount = wordCharCountMap.get(c);
      if (charCount) {
        wordCharCountMap.set(c, charCount - 1);
      } else {
        isGood = false;
      }
    }
    if (isGood) {
      count += word.length;
    }
  }

  return count;
}

console.log(countCharacters(["cat", "bt", "hat", "tree"], "atach"), 6);
console.log(
  countCharacters(["hello", "world", "leetcode"], "welldonehoneyr"),
  10
);
