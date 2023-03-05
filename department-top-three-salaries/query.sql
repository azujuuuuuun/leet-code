SELECT
    d.name AS Department,
    e.name AS Employee,
    e.salary AS Salary
FROM
    Department AS d
    JOIN Employee AS e ON d.id = e.departmentId
    JOIN (
        SELECT
            departmentId,
            SUBSTRING_INDEX(
                GROUP_CONCAT(DISTINCT salary ORDER BY salary DESC),
                ',',
                3
            ) AS salary_list
        FROM
            Employee
        GROUP BY
            departmentId
    ) AS s ON d.id = s.departmentId
        AND FIND_IN_SET(e.salary, s.salary_list) != 0;
