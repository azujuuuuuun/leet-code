function totalMoney(n: number): number {
  const week = Math.floor(n / 7);
  const dayOfWeek = n % 7;
  const base = 28 * week;
  const diff =
    week > 0 ? Math.floor(((7 + 7 * (week - 1)) * (week - 1)) / 2) : 0;
  const rest = Math.floor(((1 + week + (dayOfWeek + week)) * dayOfWeek) / 2);
  return base + diff + rest;
}

console.log(totalMoney(4), 10);
console.log(totalMoney(10), 37);
console.log(totalMoney(20), 96);
console.log(totalMoney(22), 109);
