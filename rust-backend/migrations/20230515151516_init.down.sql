-- Add down migration script here
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS bugReports;
DROP TYPE IF EXISTS bugReportStatus;