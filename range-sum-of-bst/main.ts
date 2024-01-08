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

function rangeSumBST(root: TreeNode | null, low: number, high: number): number {
  if (!root) {
    return 0;
  }
  const queue: TreeNode[] = [];
  queue.push(root);
  let sum = 0;
  while (queue.length !== 0) {
    const node = queue.shift();
    if (!node) {
      continue;
    }
    if (node.val >= low && node.val <= high) {
      sum += node.val;
    }
    if (node.left) {
      queue.push(node.left);
    }
    if (node.right) {
      queue.push(node.right);
    }
  }
  return sum;
}

console.log(
  rangeSumBST(
    new TreeNode(
      10,
      new TreeNode(5, new TreeNode(3), new TreeNode(7)),
      new TreeNode(15, null, new TreeNode(18))
    ),
    7,
    15
  ),
  32
);
console.log(
  rangeSumBST(
    new TreeNode(
      10,
      new TreeNode(
        5,
        new TreeNode(3, new TreeNode(1), null),
        new TreeNode(7, new TreeNode(6), null)
      ),
      new TreeNode(15, new TreeNode(13), new TreeNode(18))
    ),
    6,
    10
  ),
  23
);
