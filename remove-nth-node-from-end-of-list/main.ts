/**
 * Definition for singly-linked list.
 */
class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
  if (!head) {
    return null;
  }
  const stack = [head];
  let node = head;
  while (node.next !== null) {
    stack.push(node.next);
    node = node.next;
  }
  if (stack.length === n) {
    return stack[1] ?? null;
  }
  if (n === 1) {
    stack[stack.length - 2].next = null;
    return head;
  }
  stack[stack.length - 1 - n].next = stack[stack.length - 1 - n + 2];
  return head;
}

console.log(
  removeNthFromEnd(
    new ListNode(
      1,
      new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5))))
    ),
    2
  ),
  [1, 2, 3, 5]
);
console.log(removeNthFromEnd(new ListNode(1), 1), []);
console.log(removeNthFromEnd(new ListNode(1, new ListNode(2)), 1), [1]);
