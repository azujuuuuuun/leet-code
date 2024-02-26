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

function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
  if (!p && !q) {
    return true;
  } else if (p && !q) {
    return false;
  } else if (!p && q) {
    return false;
  }
  const pQueue = [p];
  const qQueue = [q];
  while (pQueue.length !== 0 && qQueue.length !== 0) {
    const pNode = pQueue.pop();
    const qNode = qQueue.pop();
    if (!pNode || !qNode) {
      return false;
    }
    if (pNode.val !== qNode.val) {
      return false;
    }
    if (pNode.left && qNode.left) {
      pQueue.push(pNode.left);
      qQueue.push(qNode.left);
    } else if (pNode.left && !qNode.left) {
      return false;
    } else if (!pNode.left && qNode.left) {
      return false;
    }
    if (pNode.right && qNode.right) {
      pQueue.push(pNode.right);
      qQueue.push(qNode.right);
    } else if (pNode.right && !qNode.right) {
      return false;
    } else if (!pNode.right && qNode.right) {
      return false;
    }
  }
  return true;
}

console.log(
  isSameTree(
    new TreeNode(1, new TreeNode(2), new TreeNode(3)),
    new TreeNode(1, new TreeNode(2), new TreeNode(3))
  ),
  true
);
console.log(
  isSameTree(
    new TreeNode(1, new TreeNode(2)),
    new TreeNode(1, undefined, new TreeNode(2))
  ),
  false
);
console.log(
  isSameTree(
    new TreeNode(1, new TreeNode(2), new TreeNode(1)),
    new TreeNode(1, new TreeNode(1), new TreeNode(2))
  ),
  false
);
