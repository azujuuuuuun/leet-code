SELECT DISTINCT
    d.id AS id,
    d1.revenue AS Jan_Revenue,
    d2.revenue AS Feb_Revenue,
    d3.revenue AS Mar_Revenue,
    d4.revenue AS Apr_Revenue,
    d5.revenue AS May_Revenue,
    d6.revenue AS Jun_Revenue,
    d7.revenue AS Jul_Revenue,
    d8.revenue AS Aug_Revenue,
    d9.revenue AS Sep_Revenue,
    d10.revenue AS Oct_Revenue,
    d11.revenue AS Nov_Revenue,
    d12.revenue AS Dec_Revenue
FROM
    Department d
    LEFT JOIN Department d1 ON d.id = d1.id AND d1.month = 'Jan'
    LEFT JOIN Department d2 ON d.id = d2.id AND d2.month = 'Feb'
    LEFT JOIN Department d3 ON d.id = d3.id AND d3.month = 'Mar'
    LEFT JOIN Department d4 ON d.id = d4.id AND d4.month = 'Apr'
    LEFT JOIN Department d5 ON d.id = d5.id AND d5.month = 'May'
    LEFT JOIN Department d6 ON d.id = d6.id AND d6.month = 'Jun'
    LEFT JOIN Department d7 ON d.id = d7.id AND d7.month = 'Jul'
    LEFT JOIN Department d8 ON d.id = d8.id AND d8.month = 'Aug'
    LEFT JOIN Department d9 ON d.id = d9.id AND d9.month = 'Sep'
    LEFT JOIN Department d10 ON d.id = d10.id AND d10.month = 'Oct'
    LEFT JOIN Department d11 ON d.id = d11.id AND d11.month = 'Nov'
    LEFT JOIN Department d12 ON d.id = d12.id AND d12.month = 'Dec';
