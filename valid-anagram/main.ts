function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) {
    return false;
  }

  const sCharCount = new Map<string, number>();
  const tCharCount = new Map<string, number>();
  for (const c of s) {
    const count = sCharCount.get(c);
    if (count) {
      sCharCount.set(c, count + 1);
    } else {
      sCharCount.set(c, 1);
    }
  }
  for (const c of t) {
    const count = tCharCount.get(c);
    if (count) {
      tCharCount.set(c, count + 1);
    } else {
      tCharCount.set(c, 1);
    }
  }

  for (const [c, sCount] of sCharCount.entries()) {
    const tCount = tCharCount.get(c);
    if (tCount) {
      if (tCount != sCount) {
        return false;
      }
    } else {
      return false;
    }
  }

  return true;
}

console.log(isAnagram("anagram", "nagaram"), true);
console.log(isAnagram("rat", "car"), false);
