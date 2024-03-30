-- Add migration script here
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
    subcategory_id INT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (subcategory_id) REFERENCES subcategories(id)
);

CREATE TABLE comments (
    id INT AUTO_INCREMENT PRIMARY KEY,
    article_id INT,
    user_id INT,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (article_id) REFERENCES articles(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE likes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    article_id INT,
    user_id INT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (article_id) REFERENCES articles(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- 初期データの投入-- カテゴリーの追加
INSERT INTO categories (name) VALUES ('物理学'), ('化学'), ('生物学');

-- サブカテゴリーの追加
INSERT INTO subcategories (name, category_id) VALUES
('量子力学', 1),
('有機化学', 2),
('進化論', 3);

-- ユーザーの追加
INSERT INTO users (username, email, password, icon_url) VALUES
('user1', 'user1@example.com', 'password1', 'https://example.com/icon1.jpg'),
('user2', 'user2@example.com', 'password2', 'https://example.com/icon2.jpg'),
('user3', 'user3@example.com', 'password3', 'https://example.com/icon3.jpg');

-- 記事の追加
INSERT INTO articles (title, content, slug, subcategory_id, user_id) VALUES
('量子力学入門', '量子力学についての基本的な説明です。', 'quantum-mechanics-intro', 1, 1),
('有機化学の基礎', '有機化学の基本的な概念や反応について解説します。', 'organic-chemistry-basics', 2, 2),
('進化論の歴史', '進化論の歴史的な背景や重要な発見についてまとめました。', 'history-of-evolutionary-theory', 3, 3);

-- コメントの追加
INSERT INTO comments (article_id, user_id, content) VALUES
(1, 2, 'わかりやすい説明で参考になりました。'),
(2, 3, 'この記事はとても勉強になります。'),
(3, 1, '進化論の歴史について興味深い内容でした。');

-- いいねの追加
INSERT INTO likes (article_id, user_id) VALUES
(1, 3),
(2, 1),
(2, 2),
(3, 2),
(3, 3);