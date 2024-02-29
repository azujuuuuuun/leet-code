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

function isEvenOddTree(root: TreeNode | null): boolean {
  if (!root) {
    return false;
  }
  const queue = [{ node: root, level: 0 }];
  let level = -1;
  let current = -1;
  while (queue.length !== 0) {
    const node = queue.shift();
    if (!node) {
      return false;
    }
    if (node.node.left) {
      queue.push({ node: node.node.left, level: node.level + 1 });
    }
    if (node.node.right) {
      queue.push({ node: node.node.right, level: node.level + 1 });
    }
    if (level !== node.level) {
      if (node.level % 2 === 0) {
        if (node.node.val % 2 === 0) {
          return false;
        }
      } else {
        if (node.node.val % 2 === 1) {
          return false;
        }
      }
      level = node.level;
      current = node.node.val;
    } else {
      if (level % 2 === 0) {
        if (node.node.val % 2 !== 1 || node.node.val <= current) {
          return false;
        }
        current = node.node.val;
      } else {
        if (node.node.val % 2 !== 0 || node.node.val >= current) {
          return false;
        }
        current = node.node.val;
      }
    }
  }
  return true;
}

console.log(
  isEvenOddTree(
    new TreeNode(
      1,
      new TreeNode(10, new TreeNode(3, new TreeNode(12), new TreeNode(8))),
      new TreeNode(
        4,
        new TreeNode(7, new TreeNode(6)),
        new TreeNode(9, null, new TreeNode(2))
      )
    )
  ),
  true
);
console.log(
  isEvenOddTree(
    new TreeNode(
      5,
      new TreeNode(4, new TreeNode(3), new TreeNode(3)),
      new TreeNode(2, new TreeNode(7))
    )
  ),
  false
);
console.log(
  isEvenOddTree(
    new TreeNode(
      5,
      new TreeNode(9, new TreeNode(3), new TreeNode(5)),
      new TreeNode(1, new TreeNode(7))
    )
  ),
  false
);
console.log(
  isEvenOddTree(
    new TreeNode(
      2,
      new TreeNode(
        12,
        new TreeNode(5, new TreeNode(18), new TreeNode(16)),
        new TreeNode(9)
      ),
      new TreeNode(8)
    )
  ),
  false
);
