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

function dfs(node: TreeNode | null): TreeNode[] {
  if (!node) {
    return [];
  }
  const nodes: TreeNode[] = [];
  if (!node.left && !node.right) {
    nodes.push(node);
  }
  if (node.left) {
    nodes.push(...dfs(node.left));
  }
  if (node.right) {
    nodes.push(...dfs(node.right));
  }
  return nodes;
}

function leafSimilar(root1: TreeNode | null, root2: TreeNode | null): boolean {
  const leaves1 = dfs(root1);
  const leaves2 = dfs(root2);
  if (leaves1.length !== leaves2.length) {
    return false;
  }
  for (let i = 0; i < leaves1.length; i++) {
    if (leaves1[i]?.val !== leaves2[i]?.val) {
      return false;
    }
  }
  return true;
}

console.log(
  leafSimilar(
    new TreeNode(
      3,
      new TreeNode(
        5,
        new TreeNode(6),
        new TreeNode(2, new TreeNode(7), new TreeNode(4))
      ),
      new TreeNode(1, new TreeNode(9), new TreeNode(8))
    ),
    new TreeNode(
      3,
      new TreeNode(5, new TreeNode(6), new TreeNode(7)),
      new TreeNode(
        1,
        new TreeNode(4),
        new TreeNode(2, new TreeNode(9), new TreeNode(8))
      )
    )
  ),
  true
);
console.log(
  leafSimilar(
    new TreeNode(1, new TreeNode(2), new TreeNode(3)),
    new TreeNode(1, new TreeNode(3), new TreeNode(2))
  ),
  false
);
