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

function findBottomLeftValue(root: TreeNode | null): number {
  if (!root) {
    return 0;
  }

  const queue = [{ node: root, depth: 0 }];
  let maxDepth = -1;
  let ans = 0;
  while (queue.length !== 0) {
    const node = queue.shift();
    if (!node) {
      continue;
    }
    if (node.depth > maxDepth) {
      ans = node.node.val;
      maxDepth = node.depth;
    }
    if (node.node.left) {
      queue.push({ node: node.node.left, depth: node.depth + 1 });
    }
    if (node.node.right) {
      queue.push({
        node: node.node.right,
        depth: node.depth + 1,
      });
    }
  }

  return ans;
}

console.log(
  findBottomLeftValue(new TreeNode(2, new TreeNode(1), new TreeNode(3))),
  1
);
console.log(
  findBottomLeftValue(
    new TreeNode(
      1,
      new TreeNode(2, new TreeNode(4)),
      new TreeNode(3, new TreeNode(5, new TreeNode(7)), new TreeNode(6))
    )
  ),
  7
);
console.log(findBottomLeftValue(new TreeNode(1, null, new TreeNode(1))), 1);
console.log(findBottomLeftValue(new TreeNode(0, null, new TreeNode(-1))), -1);
