SELECT
    total.request_at AS Day,
    (
        CASE WHEN total.cnt IS NULL OR total.cnt = 0 THEN 0
            WHEN cancel.cnt IS NULL OR cancel.cnt = 0 THEN 0
            ELSE ROUND(cancel.cnt / total.cnt, 2)
        END
    ) AS `Cancellation Rate`
FROM
    (
        SELECT
            request_at,
            COUNT(*) AS cnt
        FROM
            Trips AS t
            LEFT OUTER JOIN Users AS c ON t.client_id = c.users_id
            LEFT OUTER JOIN Users AS d ON t.driver_id = d.users_id
        WHERE
            t.request_at BETWEEN "2013-10-01" AND "2013-10-03"
            AND c.banned = "No"
            AND d.banned = "No"
        GROUP BY
            request_at
    ) AS total
    LEFT OUTER JOIN (
        SELECT
            request_at,
            COUNT(*) AS cnt
        FROM
            Trips AS t
            LEFT OUTER JOIN Users AS c ON t.client_id = c.users_id
            LEFT OUTER JOIN Users AS d ON t.driver_id = d.users_id
        WHERE
            t.request_at BETWEEN "2013-10-01" AND "2013-10-03"
            AND c.banned = "No"
            AND d.banned = "No"
            AND t.status IN ("cancelled_by_client", "cancelled_by_driver")
        GROUP BY
            request_at
    ) AS cancel ON total.request_at = cancel.request_at;
