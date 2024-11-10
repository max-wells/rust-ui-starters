-- Add migration script here

-- Recreate tables with the new schema
CREATE TABLE IF NOT EXISTS blogs (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    author VARCHAR(100) NOT NULL,
    image_url VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS blog_tags (
    blog_id INT,
    tag_id INT,
    PRIMARY KEY (blog_id, tag_id),
    FOREIGN KEY (blog_id) REFERENCES blogs(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Insert some sample data
INSERT INTO blogs (title, content, author, image_url) VALUES
('First Blog Post', 'This is the content of the first blog post.', 'John Doe', 'https://proedu.com/cdn/shop/articles/What-Is-Grayscale-Photshop-Skills-blog.jpg?v=1702337949&width=1400'),
('Second Blog Post', 'This is the content of the second blog post.', 'Jane Smith', 'https://proedu.com/cdn/shop/articles/What-Is-Grayscale-Photshop-Skills-blog.jpg?v=1702337949&width=1400')
ON CONFLICT (id) DO NOTHING;

INSERT INTO tags (name) VALUES
('Technology'),
('Programming'),
('Rust'),
('Web Development')
ON CONFLICT (name) DO NOTHING;

-- For blog_tags, we need to get the actual IDs of blogs and tags
DO $$
DECLARE
    first_blog_id INT;
    second_blog_id INT;
    technology_tag_id INT;
    programming_tag_id INT;
    rust_tag_id INT;
    web_dev_tag_id INT;
BEGIN
    SELECT id INTO first_blog_id FROM blogs WHERE title = 'First Blog Post' LIMIT 1;
    SELECT id INTO second_blog_id FROM blogs WHERE title = 'Second Blog Post' LIMIT 1;
    SELECT id INTO technology_tag_id FROM tags WHERE name = 'Technology' LIMIT 1;
    SELECT id INTO programming_tag_id FROM tags WHERE name = 'Programming' LIMIT 1;
    SELECT id INTO rust_tag_id FROM tags WHERE name = 'Rust' LIMIT 1;
    SELECT id INTO web_dev_tag_id FROM tags WHERE name = 'Web Development' LIMIT 1;

    INSERT INTO blog_tags (blog_id, tag_id) VALUES
    (first_blog_id, technology_tag_id),
    (first_blog_id, programming_tag_id),
    (first_blog_id, rust_tag_id),
    (second_blog_id, programming_tag_id),
    (second_blog_id, web_dev_tag_id)
    ON CONFLICT (blog_id, tag_id) DO NOTHING;
END $$;