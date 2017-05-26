-- Your SQL goes here
CREATE TABLE entries (
  id         INTEGER NOT NULL PRIMARY KEY,
  date       TEXT    NOT NULL,

  food       INTEGER NOT NULL REFERENCES foods(id),
  unit       INTEGER REFERENCES units(id),
  quantity   REAL    NOT NULL
);

CREATE TABLE units (
  id         INTEGER NOT NULL PRIMARY KEY,
  name       TEXT    NOT NULL,
  grams      REAL    NOT NULL,
  food       INTEGER REFERENCES foods(id)
)
