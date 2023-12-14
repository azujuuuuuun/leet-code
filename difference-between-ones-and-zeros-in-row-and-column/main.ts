function onesMinusZeros(grid: number[][]): number[][] {
  const onesRow: number[] = [];
  const onesCol: number[] = [];
  const zerosRow: number[] = [];
  const zerosCol: number[] = [];
  for (let i = 0; i < grid.length; i++) {
    let oneCount = 0;
    for (let j = 0; j < grid[i].length; j++) {
      if (grid[i][j] === 1) {
        oneCount++;
      }
    }
    onesRow.push(oneCount);
    zerosRow.push(grid[i].length - oneCount);
  }
  for (let i = 0; i < grid[0].length; i++) {
    let oneCount = 0;
    for (let j = 0; j < grid.length; j++) {
      if (grid[j][i] === 1) {
        oneCount++;
      }
    }
    onesCol.push(oneCount);
    zerosCol.push(grid.length - oneCount);
  }
  const diff: number[][] = [];
  for (let i = 0; i < grid.length; i++) {
    const row: number[] = [];
    for (let j = 0; j < grid[i].length; j++) {
      row.push(onesRow[i] + onesCol[j] - zerosRow[i] - zerosCol[j]);
    }
    diff.push(row);
  }
  return diff;
}

console.log(
  onesMinusZeros([
    [0, 1, 1],
    [1, 0, 1],
    [0, 0, 1],
  ]),
  [
    [0, 0, 4],
    [0, 0, 4],
    [-2, -2, 2],
  ]
);
console.log(
  onesMinusZeros([
    [1, 1, 1],
    [1, 1, 1],
  ]),
  [
    [5, 5, 5],
    [5, 5, 5],
  ]
);
