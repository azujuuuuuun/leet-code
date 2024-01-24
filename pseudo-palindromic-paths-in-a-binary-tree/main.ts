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

function isPseudoPalindromicPath(map: Map<number, number>): boolean {
  let oddCount = 0;
  map.forEach((v) => {
    if (v % 2 == 1) {
      oddCount++;
    }
  });
  return oddCount <= 1;
}

function dfs(node: TreeNode | null, map: Map<number, number>): number {
  if (!node) {
    return 0;
  }

  const currentMap = new Map<number, number>();
  map.forEach((v, k) => {
    currentMap.set(k, v);
  });

  const count = currentMap.get(node.val);
  if (count) {
    currentMap.set(node.val, count + 1);
  } else {
    currentMap.set(node.val, 1);
  }

  if (!node.left && !node.right) {
    if (isPseudoPalindromicPath(currentMap)) {
      return 1;
    } else {
      return 0;
    }
  }

  let sum = 0;
  if (node.left) {
    sum += dfs(node.left, currentMap);
  }
  if (node.right) {
    sum += dfs(node.right, currentMap);
  }
  return sum;
}

function pseudoPalindromicPaths(root: TreeNode | null): number {
  return dfs(root, new Map());
}

console.log(
  pseudoPalindromicPaths(
    new TreeNode(
      2,
      new TreeNode(3, new TreeNode(3), new TreeNode(1)),
      new TreeNode(1, null, new TreeNode(1))
    )
  ),
  2
);
console.log(
  pseudoPalindromicPaths(
    new TreeNode(
      2,
      new TreeNode(1, new TreeNode(1), new TreeNode(3, null, new TreeNode(1))),
      new TreeNode(1)
    )
  ),
  1
);
console.log(pseudoPalindromicPaths(new TreeNode(9)), 1);
