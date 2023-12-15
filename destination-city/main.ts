function destCity(paths: string[][]): string {
  const map = new Map<string, boolean>();
  for (const [cityA, cityB] of paths) {
    map.set(cityA, true);
    const outgoing = map.get(cityB);
    if (!outgoing) {
      map.set(cityB, false);
    }
  }
  for (const [city, outgoing] of map.entries()) {
    if (!outgoing) {
      return city;
    }
  }
  return "";
}

console.log(
  destCity([
    ["London", "New York"],
    ["New York", "Lima"],
    ["Lima", "Sao Paulo"],
  ]),
  "Sao Paulo"
);
console.log(
  destCity([
    ["B", "C"],
    ["D", "B"],
    ["C", "A"],
  ]),
  "A"
);
console.log(destCity([["A", "Z"]]), "Z");
