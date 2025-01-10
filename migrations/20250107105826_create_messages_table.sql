-- Add migration script here
CREATE TABLE messages (
                          id SERIAL PRIMARY KEY,
                          content TEXT NOT NULL,
                          author VARCHAR(255) NOT NULL,
                          created_at TIMESTAMP DEFAULT NOW()
);