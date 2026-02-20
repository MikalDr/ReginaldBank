UPDATE "holdings" 
SET count = CASE item
    WHEN 'pp' THEN :pp
    WHEN 'ep' THEN :ep
    WHEN 'gp' THEN :gp
    WHEN 'sp' THEN :sp
    WHEN 'cp' THEN :cp
    ELSE count
END
WHERE item IN ('pp', 'ep', 'gp', 'sp', 'cp');