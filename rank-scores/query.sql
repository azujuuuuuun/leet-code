SELECT
    s1.score,
    (
        SELECT
            COUNT(*) + 1
        FROM (
            SELECT
                COUNT(s2.score) + 1
            FROM
                Scores AS s2
            WHERE
                s2.score > s1.score
            GROUP BY
                s2.score
        ) AS s
    ) AS `rank`
FROM
    Scores AS s1
ORDER BY
    s1.score DESC
