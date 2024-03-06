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

function hasCycle(head: ListNode | null): boolean {
  let fast = head;
  let slow = head;
  while (fast !== null && fast.next !== null) {
    fast = fast.next.next;
    slow = slow?.next ?? null;
    if (fast === slow) {
      return true;
    }
  }
  return false;
}

const node3 = new ListNode(-4);
const node2 = new ListNode(0, node3);
let node1 = new ListNode(2, node2);
let head = new ListNode(3, node1);
node3.next = node1;
console.log(hasCycle(head), true);

node1 = new ListNode(2);
head = new ListNode(1, node1);
node1.next = head;
console.log(hasCycle(head), true);

head = new ListNode(1);
console.log(hasCycle(head), false);
