-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  unionID VARCHAR NOT NULL,
  platform VARCHAR NOT NULL,
  openid VARCHAR NOT NULL UNIQUE,
  name VARCHAR NOT NULL,
  derive VARCHAR NOT NULL,
  out_ip VARCHAR NOT NULL,
  in_ip VARCHAR NOT NULL,
  blank BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE TABLE users_extra (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  first_launch_path VARCHAR NOT NULL,
  first_launch_scene VARCHAR NOT NULL
);

CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  blank BOOLEAN NOT NULL DEFAULT FALSE,
  creator_id INTEGER NOT NULL,  -- user_id
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE TABLE resources (
  id SERIAL PRIMARY KEY,
  room_id INTEGER NOT NULL,
  name VARCHAR NOT NULL,
  size INTEGER NOT NULL, -- in bytes
  key VARCHAR NOT NULL,
  length INTEGER NOT NULL, -- in seconds
  creator_id INTEGER NOT NULL,  -- user_id
  down_count INTEGER NOT NULL DEFAULT 0,
  blank BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP
);

CREATE TABLE downlogs (
  id SERIAL PRIMARY KEY,
  resource_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  status INTEGER NOT NULL DEFAULT 0, -- 0: downloading, 1: downloaded
  created_at TIMESTAMP NOT NULL DEFAULT NOW()
);