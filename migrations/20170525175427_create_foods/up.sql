-- Your SQL goes here
CREATE TABLE foods (
  id       INTEGER NOT NULL PRIMARY KEY,
  name     VARCHAR NOT NULL,

  calories REAL    NOT NULL,

  lipids        REAL    NOT NULL,
  protein       REAL    NOT NULL,
  carbohydrates REAL    NOT NULL
)
