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

function dfs(node: TreeNode | null): string {
  if (node === null) {
    return "";
  }
  if (node.left !== null && node.right !== null) {
    return `${node.val}(${dfs(node.left)})(${dfs(node.right)})`;
  }
  if (node.left !== null && node.right === null) {
    return `${node.val}(${dfs(node.left)})`;
  }
  if (node.left === null && node.right !== null) {
    return `${node.val}()(${dfs(node.right)})`;
  }
  if (node.left === null && node.right === null) {
    return `${node.val}`;
  }
  return "";
}

function tree2str(root: TreeNode | null): string {
  return dfs(root);
}

console.log(
  tree2str(new TreeNode(1, new TreeNode(2, new TreeNode(4)), new TreeNode(3))),
  "1(2(4))(3)"
);
console.log(
  tree2str(
    new TreeNode(1, new TreeNode(2, null, new TreeNode(4)), new TreeNode(3))
  ),
  "1(2()(4))(3)"
);
