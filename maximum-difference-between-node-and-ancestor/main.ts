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

function dfs(node: TreeNode, nodeValues: number[]): number {
  let max = 0;
  nodeValues.forEach((v) => {
    const diff = Math.abs(v - node.val);
    if (diff > max) {
      max = diff;
    }
  });
  if (node.left) {
    const leftMax = dfs(node.left, [...nodeValues, node.val]);
    if (leftMax > max) {
      max = leftMax;
    }
  }
  if (node.right) {
    const rightMax = dfs(node.right, [...nodeValues, node.val]);
    if (rightMax > max) {
      max = rightMax;
    }
  }
  return max;
}

function maxAncestorDiff(root: TreeNode | null): number {
  if (!root) {
    return 0;
  }
  return dfs(root, []);
}

console.log(
  maxAncestorDiff(
    new TreeNode(
      8,
      new TreeNode(
        3,
        new TreeNode(1),
        new TreeNode(6, new TreeNode(4), new TreeNode(7))
      ),
      new TreeNode(10, null, new TreeNode(14, new TreeNode(13)))
    )
  ),
  7
);
console.log(
  maxAncestorDiff(
    new TreeNode(
      1,
      null,
      new TreeNode(2, null, new TreeNode(0, new TreeNode(3)))
    )
  ),
  3
);
