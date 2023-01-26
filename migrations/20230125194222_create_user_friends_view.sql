CREATE VIEW
  user_friends AS
SELECT
  first_user_id AS user_id,
  second_user_id AS friend_id
FROM
  friendships
UNION ALL
SELECT
  second_user_id AS user_id,
  first_user_id AS friend_id
FROM
  friendships
