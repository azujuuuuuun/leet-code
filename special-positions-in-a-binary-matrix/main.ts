function numSpecial(mat: number[][]): number {
  const countRow: number[] = [];
  const countColumn: number[] = [];
  for (let i = 0; i < mat.length; i++) {
    let count = 0;
    for (let j = 0; j < mat[i].length; j++) {
      if (mat[i][j] === 1) {
        count++;
      }
    }
    countRow.push(count);
  }
  for (let i = 0; i < mat[0].length; i++) {
    let count = 0;
    for (let j = 0; j < mat.length; j++) {
      if (mat[j][i] === 1) {
        count++;
      }
    }
    countColumn.push(count);
  }
  let count = 0;
  for (let i = 0; i < mat.length; i++) {
    for (let j = 0; j < mat[i].length; j++) {
      if (mat[i][j] === 1 && countRow[i] === 1 && countColumn[j] === 1) {
        count++;
      }
    }
  }
  return count;
}

console.log(
  numSpecial([
    [1, 0, 0],
    [0, 0, 1],
    [1, 0, 0],
  ]),
  1
);
console.log(
  numSpecial([
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
  ]),
  3
);
