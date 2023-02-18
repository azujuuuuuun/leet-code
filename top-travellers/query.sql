SELECT
    u.name AS name,
    (CASE
        WHEN SUM(r.distance) IS NULL THEN 0
        ELSE SUM(r.distance)
    END) AS travelled_distance
FROM Users AS u
LEFT OUTER JOIN Rides AS r
    ON u.id = r.user_id
GROUP BY u.id
ORDER BY SUM(r.distance) DESC, u.name ASC;