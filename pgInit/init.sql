-- Users table for authentication
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    user_name VARCHAR(100),
    user_email VARCHAR(255) UNIQUE NOT NULL,
    user_password_hash VARCHAR(255) NOT NULL,    
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


-- Basic todos table
CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    todo_text TEXT,
    completed BOOLEAN DEFAULT FALSE NOT NULL,
    priority INTEGER DEFAULT 1 NOT NULL, -- 1 (low) to 5 (high)
    due_date TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id INTEGER REFERENCES users(user_id) NOT NULL
);


-- Optional: Tags/Categories for todos
CREATE TABLE tags (
    tag_id SERIAL PRIMARY KEY,
    tag_name VARCHAR(50) NOT NULL,
    user_id INTEGER REFERENCES users(user_id)
);

-- Junction table for many-to-many relationship between todos and tags
CREATE TABLE todo_tags (
    todo_id INTEGER REFERENCES todos(todo_id) ON DELETE CASCADE,
    tag_id INTEGER REFERENCES tags(tag_id) ON DELETE CASCADE,
    PRIMARY KEY (todo_id, tag_id)
);

-- Create indexes for better query performance
CREATE INDEX idx_todos_user_id ON todos(user_id);
CREATE INDEX idx_todos_completed ON todos(completed);
CREATE INDEX idx_tags_user_id ON tags(user_id);


-- Test/Demo data !
insert into users (user_id, user_name, user_email, user_password_hash)
    values (1, 'Demo User 1', 'DemoUser1@nowhere', 'n/a');
commit;