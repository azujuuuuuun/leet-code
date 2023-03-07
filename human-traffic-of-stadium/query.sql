SELECT
    s.id AS id,
    s.visit_date AS visit_date,
    s.people AS people
FROM
    Stadium AS s
    LEFT OUTER JOIN Stadium AS yy ON s.id - 2 = yy.id
    LEFT OUTER JOIN Stadium AS y ON s.id - 1 = y.id
    LEFT OUTER JOIN Stadium AS t ON s.id + 1 = t.id
    LEFT OUTER JOIN Stadium AS tt ON s.id + 2 = tt.id
WHERE
    (yy.people >= 100 AND y.people >= 100 AND s.people >= 100)
    OR (y.people >= 100 AND s.people >= 100 AND t.people >= 100)
    OR (s.people >= 100 AND t.people >= 100 AND tt.people >= 100);
