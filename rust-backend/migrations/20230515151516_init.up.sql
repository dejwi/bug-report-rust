-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
  IF NOT EXISTS users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username VARCHAR(30) NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at TIMESTAMP 
      WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TYPE bugReportStatus AS ENUM ('OPEN', 'CLOSED', 'SOLVED', 'REVIEW', 'ACCEPTED');

CREATE TABLE 
  IF NOT EXISTS bugReports (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status bugReportStatus NOT NULL DEFAULT 'OPEN',
    author_id UUID NOT NULL REFERENCES users (id),
    created_at TIMESTAMP 
      WITH TIME ZONE NOT NULL DEFAULT NOW()
);