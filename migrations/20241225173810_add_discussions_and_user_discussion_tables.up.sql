-- Add up migration script here
CREATE TABLE discussions (
     id INT AUTO_INCREMENT PRIMARY KEY,
     created_by INT NOT NULL,
     date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
     FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE NO ACTION
);

CREATE TABLE user_discussion (
     id INT AUTO_INCREMENT PRIMARY KEY,
     user_id INT NOT NULL,
     discussion_id INT NOT NULL,
     date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
     FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
     FOREIGN KEY (discussion_id) REFERENCES discussions(id) ON DELETE CASCADE
);

CREATE TABLE messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    discussion_id INT NOT NULL,
    sender_id INT NOT NULL,
    text VARCHAR(1000) NOT NULL,
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (discussion_id) REFERENCES discussions(id) ON DELETE CASCADE,
    FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE NO ACTION
);
