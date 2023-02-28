SELECT DISTINCT
    t.id,
    CASE
        WHEN t.p_id IS NULL THEN "Root"
        WHEN c.id IS NOT NULL THEN "Inner"
        ELSE "Leaf"
    END AS type
FROM
    Tree AS t
    LEFT OUTER JOIN Tree AS c ON t.id = c.p_id;
