SELECT
    d.name AS Department,
    e.name AS Employee,
    salary AS Salary
FROM
    Employee AS e
    JOIN (
        SELECT
            d2.id AS id,
            d2.name AS name,
            MAX(salary) AS max_salary
        FROM
            Employee AS e2
            JOIN Department AS d2 ON e2.departmentId = d2.id
        GROUP BY
            departmentId
    ) AS d ON e.departmentId = d.id
WHERE
    e.salary = d.max_salary;
