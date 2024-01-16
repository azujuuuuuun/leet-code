class RandomizedSet {
  arr: number[];
  map: Map<number, number>;

  constructor() {
    this.arr = [];
    this.map = new Map();
  }

  insert(val: number): boolean {
    if (this.map.has(val)) {
      return false;
    }

    this.arr.push(val);
    this.map.set(val, this.arr.length - 1);

    return true;
  }

  remove(val: number): boolean {
    if (!this.map.has(val)) {
      return false;
    }

    const index = this.map.get(val);
    if (typeof index === "number") {
      this.arr[index] = this.arr[this.arr.length - 1];
      this.map.set(this.arr[index], index);
      this.arr.pop();
      this.map.delete(val);
    }

    return true;
  }

  getRandom(): number {
    return this.arr[Math.floor(Math.random() * this.arr.length)];
  }
}

const randomizedSet = new RandomizedSet();
console.log(randomizedSet.insert(1), true);
console.log(randomizedSet.remove(2), false);
console.log(randomizedSet.insert(2), true);
console.log(randomizedSet.getRandom(), 2);
console.log(randomizedSet.remove(1), true);
console.log(randomizedSet.insert(2), false);
console.log(randomizedSet.getRandom(), 2);

const randomizedSet2 = new RandomizedSet();
console.log(null);
console.log(randomizedSet2.remove(0), false);
console.log(randomizedSet2.remove(0), false);
console.log(randomizedSet2.insert(0), true);
console.log(randomizedSet2.getRandom(), 0);
console.log(randomizedSet2.remove(0), true);
console.log(randomizedSet2.insert(0), true);
