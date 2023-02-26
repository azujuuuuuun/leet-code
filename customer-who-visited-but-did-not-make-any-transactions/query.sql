SELECT
    customer_id,
    COUNT(v.visit_id) AS count_no_trans
FROM
    Visits AS v
    LEFT OUTER JOIN Transactions AS t ON v.visit_id = t.visit_id
WHERE
    amount IS NULL
GROUP BY
    customer_id;
