-- Add down migration script here
-- Drop the 'messages' table first because it depends on 'discussions' and 'users'
DROP TABLE IF EXISTS messages;

-- Drop the 'user_discussion' table because it depends on 'users' and 'discussions'
DROP TABLE IF EXISTS user_discussion;

-- Drop the 'discussions' table
DROP TABLE IF EXISTS discussions;
