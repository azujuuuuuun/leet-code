SELECT
    user_id AS buyer_id,
    join_date,
    COUNT(
        CASE WHEN order_date BETWEEN '2019-01-01' AND '2019-12-31' THEN 1 ELSE NULL END
    ) AS orders_in_2019
FROM
    Users
    LEFT OUTER JOIN Orders ON user_id = buyer_id
GROUP BY
    user_id;
