-- Add migration script here
CREATE TABLE messages (
                          id SERIAL PRIMARY KEY,
                          content TEXT NOT NULL,
                          username VARCHAR(255) NOT NULL,
                          created_at TIMESTAMP DEFAULT NOW()
);