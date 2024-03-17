function splitWordsBySeparator(words: string[], separator: string): string[] {
  return words
    .reduce((prev, cur) => prev.concat(cur.split(separator)), [] as string[])
    .filter((word) => word);
}

console.log(splitWordsBySeparator(["one.two.three", "four.five", "six"], "."), [
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
]);
console.log(splitWordsBySeparator(["$easy$", "$problem$"], "$"), [
  "easy",
  "problem",
]);
console.log(splitWordsBySeparator(["|||"], "|"), []);
