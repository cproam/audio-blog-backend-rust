CREATE TABLE images (
    id UUID PRIMARY KEY,
    url VARCHAR(255) NOT NULL,
    alt TEXT,
    article_id UUID,
    FOREIGN KEY (article_id) REFERENCES articles(id) ON DELETE CASCADE
);