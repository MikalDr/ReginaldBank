SELECT
  item,
  count
FROM
  holdings
WHERE
  item = "PP"
  OR ITEM = "EP"
  OR item = "GP"
  OR item = "SP"
  OR item = "CP"