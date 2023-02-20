/*
Input:
Employees table:
+-------------+----------+
| employee_id | name     |
+-------------+----------+
| 2           | Crew     |
| 4           | Haven    |
| 5           | Kristian |
+-------------+----------+
Salaries table:
+-------------+--------+
| employee_id | salary |
+-------------+--------+
| 5           | 76071  |
| 1           | 22517  |
| 4           | 63539  |
+-------------+--------+
Output:
+-------------+
| employee_id |
+-------------+
| 1           |
| 2           |
+-------------+
*/
SELECT employee_id
FROM (
    SELECT
        e.employee_id AS employee_id
    FROM
        Employees AS e
        LEFT OUTER JOIN Salaries AS s ON e.employee_id = s.employee_id
    WHERE
        s.salary IS NULL
    UNION
    SELECT
        s.employee_id AS employee_id
    FROM
        Salaries AS s
        LEFT OUTER JOIN Employees AS e ON s.employee_id = e.employee_id
    WHERE
        e.name IS NULL
) AS u
ORDER BY employee_id;
