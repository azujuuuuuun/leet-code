SELECT DISTINCT
    l1.num AS ConsecutiveNums
FROM
    Logs AS l1
    LEFT JOIN Logs AS l2 ON l2.id = l1.id + 1 AND l1.num = l2.num
    LEFT JOIN Logs AS l3 ON l3.id = l1.id + 2 AND l1.num = l3.num
WHERE
    l1.num IS NOT NULL
    AND l2.num IS NOT NULL
    AND l3.num IS NOT NULL;
