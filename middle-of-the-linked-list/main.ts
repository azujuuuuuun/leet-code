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

function middleNode(head: ListNode | null): ListNode | null {
  let nodeNum = 0;
  let node = head;
  while (node !== null) {
    node = node.next;
    nodeNum++;
  }
  node = head;
  let count = 0;
  while (count !== Math.floor(nodeNum / 2) && node !== null) {
    node = node.next;
    count++;
  }
  return node;
}

const node4 = new ListNode(5);
const node3 = new ListNode(4, node4);
const node2 = new ListNode(3, node3);
const node1 = new ListNode(2, node2);
const head = new ListNode(1, node1);
console.log(middleNode(head), node2);

const node5 = new ListNode(6);
node4.next = node5;
console.log(middleNode(head), node3);

head.next = null;
console.log(middleNode(head), head);
