/**
 * Definition for Employee.
 */
class Employee {
  id: number;
  importance: number;
  subordinates: number[];
  constructor(id: number, importance: number, subordinates: number[]) {
    this.id = id === undefined ? 0 : id;
    this.importance = importance === undefined ? 0 : importance;
    this.subordinates = subordinates === undefined ? [] : subordinates;
  }
}

function getImportance(employees: Employee[], id: number): number {
  const map = new Map<number, Employee>();
  employees.forEach((employee) => {
    map.set(employee.id, employee);
  });

  const employee = map.get(id);
  const queue = employee ? [employee] : [];
  let total = 0;
  while (queue.length !== 0) {
    const employee = queue.pop();
    total += employee?.importance ?? 0;
    employee?.subordinates.forEach((subordinateId) => {
      const subordinate = map.get(subordinateId);
      if (subordinate) {
        queue.push(subordinate);
      }
    });
  }

  return total;
}

console.log(
  getImportance(
    [
      new Employee(1, 5, [2, 3]),
      new Employee(2, 3, []),
      new Employee(3, 3, []),
    ],
    1
  ),
  11
);

console.log(
  getImportance([new Employee(1, 2, [5]), new Employee(5, -3, [])], 5),
  -3
);
