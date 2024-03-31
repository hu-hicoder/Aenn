-- -- Add migration script here
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    icon_url VARCHAR(255),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE categories (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE subcategories (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    category_id INT,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE articles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    slug VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE article_subcategory (
    article_id INT,
    subcategory_id INT,
    PRIMARY KEY (article_id, subcategory_id),
    FOREIGN KEY (article_id) REFERENCES articles(id),
    FOREIGN KEY (subcategory_id) REFERENCES subcategories(id)
);

CREATE TABLE comments (
    id INT AUTO_INCREMENT PRIMARY KEY,
    article_id INT NOT NULL,
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (article_id) REFERENCES articles(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE likes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    article_id INT,
    user_id INT,
    FOREIGN KEY (article_id) REFERENCES articles(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- users テーブルの初期データ
INSERT INTO users (username, email, password, icon_url, created_at) VALUES
('jijinbei', 'jijinbei@example.com', '246911', 'https://example.com/icon1.png', NOW()),
('mikayu', 'mikayu@example.com', '31415926535', 'https://example.com/icon2.png', NOW()),
('user3', 'user3@example.com', 'password3', 'https://example.com/icon3.png', NOW());

-- categories テーブルの初期データ
INSERT INTO categories (name) VALUES
('物理'),
('化学'),
('生物'),
('地学'),
('数学'),
('工学');

-- subcategories テーブルの初期データ
INSERT INTO subcategories (name, category_id) VALUES
('場の量子論', 1),
('相対性理論', 1),
('古典力学',1),
('電磁気学', 1),
('有機化学', 2),
('無機化学', 2),
('生物学基礎', 3),
('生物学応用', 3),
('地質学', 4),
('気象学', 4),
('微分積分', 5),
('線形代数', 5),
('機械工学', 6),
('電気工学', 6);

-- articles テーブルの初期データ
INSERT INTO articles (title, content, slug, user_id, created_at, updated_at) VALUES
('この世は量子力学でできている', '量子力学では粒子をドブロイ波として扱う', '5de294e1-ef3a-11ee-a436-00155d3cfd23', 1, NOW(), NOW()),
('Article 2', 'Content of article 2', '36012c2c-ef3a-11ee-a436-00155d3cfd23', 2, NOW(), NOW()),
('Article 3', 'Content of article 3', '67019fa0-ef3a-11ee-a436-00155d3cfd23', 3, NOW(), NOW());

-- article_subcategory テーブルの初期データ
INSERT INTO article_subcategory (article_id, subcategory_id) VALUES
(1, 1),
(1, 2),
(2, 3),
(2, 4),
(3, 5);

-- comments テーブルの初期データ
INSERT INTO comments (article_id, user_id, content, created_at) VALUES
(1, 1, '嘘乙', NOW()),
(1, 2, '量子力学的習慣術を試したらQOL爆上がり', NOW()),
(1, 3, '. <-- これ原子', NOW()),
(2, 1, 'Comment 1 for article 2', NOW());

-- likes テーブルの初期データ
INSERT INTO likes (article_id, user_id) VALUES
(1, 1),
(1, 2),
(2, 1),
(2, 2);
