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

function dfs(node: TreeNode | null, isLeft: boolean): number {
  if (!node) {
    return 0;
  }
  if (!node.left && !node.right) {
    return isLeft ? node.val : 0;
  }
  return dfs(node.left, true) + dfs(node.right, false);
}

function sumOfLeftLeaves(root: TreeNode | null): number {
  return dfs(root, false);
}

console.log(
  sumOfLeftLeaves(
    new TreeNode(
      3,
      new TreeNode(9),
      new TreeNode(20, new TreeNode(15), new TreeNode(7))
    )
  ),
  24
);
console.log(sumOfLeftLeaves(new TreeNode(1)), 0);
