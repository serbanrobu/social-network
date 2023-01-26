CREATE TABLE
  friendships (
    first_user_id INTEGER NOT NULL REFERENCES users (id),
    second_user_id INTEGER NOT NULL REFERENCES users (id),
    PRIMARY KEY (first_user_id, second_user_id),

    -- This will prevent the relationship from being inserted in the opposite direction
    CHECK (first_user_id < second_user_id)
  );
