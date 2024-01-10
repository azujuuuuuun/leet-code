/**
 * Definition for a binary tree node.
 */
class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}

// key: val, value: valuesの隣接ノードマップと
// key: val: value: timeのマップを作る
// startから隣接ノードマップを辿り、timeを更新していく
// nodeがstartかtimeが0でなければ（訪問済みでなければ）辿る
// timeの最大値を返す
function amountOfTime(root: TreeNode | null, start: number): number {
  if (!root) {
    return 0;
  }
  const queue: TreeNode[] = [];
  queue.push(root);
  const adjacent: { [val: number]: number[] } = {};
  const time: { [val: number]: number } = {};
  while (queue.length !== 0) {
    const node = queue.shift();
    if (!node) {
      continue;
    }
    time[node.val] = 0;
    if (node.left) {
      queue.push(node.left);
      const parentNodeValues = adjacent[node.val];
      if (parentNodeValues && !parentNodeValues.includes(node.left.val)) {
        adjacent[node.val] = [...parentNodeValues, node.left.val];
      } else {
        adjacent[node.val] = [node.left.val];
      }
      const childNodeValues = adjacent[node.left.val];
      if (childNodeValues && !childNodeValues.includes(node.val)) {
        adjacent[node.left.val] = [...childNodeValues, node.val];
      } else {
        adjacent[node.left.val] = [node.val];
      }
    }
    if (node.right) {
      queue.push(node.right);
      const parentNodeValues = adjacent[node.val];
      if (parentNodeValues && !parentNodeValues.includes(node.right.val)) {
        adjacent[node.val] = [...parentNodeValues, node.right.val];
      } else {
        adjacent[node.val] = [node.right.val];
      }
      const childNodeValues = adjacent[node.right.val];
      if (childNodeValues && !childNodeValues.includes(node.val)) {
        adjacent[node.right.val] = [...childNodeValues, node.val];
      } else {
        adjacent[node.right.val] = [node.val];
      }
    }
  }

  let values = adjacent[start];
  let now = 0;
  while (values && values.length != 0) {
    now++;
    const nextValues: number[] = [];
    for (const val of values) {
      if (val != start && time[val] == 0) {
        time[val] = now;
        nextValues.push(...adjacent[val]);
      }
    }
    values = nextValues;
  }

  let max = 0;
  for (const cost of Object.values(time)) {
    if (cost > max) {
      max = cost;
    }
  }
  return max;
}

console.log(
  amountOfTime(
    new TreeNode(
      1,
      new TreeNode(5, null, new TreeNode(4, new TreeNode(9), new TreeNode(2))),
      new TreeNode(3, new TreeNode(10), new TreeNode(6))
    ),
    3
  ),
  4
);
console.log(amountOfTime(new TreeNode(1), 1), 0);
console.log(
  amountOfTime(
    new TreeNode(
      1,
      null,
      new TreeNode(
        2,
        new TreeNode(3, new TreeNode(5), new TreeNode(6)),
        new TreeNode(4, new TreeNode(7), new TreeNode(8))
      )
    ),
    8
  ),
  4
);
