CREATE TABLE profile (
    profile_id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(1024) NOT NULL,
    email_address VARCHAR(128) NOT NULL,
    join_datetime DATETIME NOT NULL,
    profile_picture VARCHAR(64)
);

CREATE TABLE private_message (
    private_message_id INT AUTO_INCREMENT PRIMARY KEY,
    sender_id INT NOT NULL,
    recipient_id INT NOT NULL,
    CONSTRAINT fk_sender
        FOREIGN KEY(sender_id) REFERENCES profile(profile_id),
    CONSTRAINT fk_recipient
        FOREIGN KEY(recipient_id) REFERENCES profile(profile_id)
);

CREATE TABLE group_chat (
    group_chat_id INT AUTO_INCREMENT PRIMARY KEY,
    creation_date DATETIME NOT NULL,
    group_picture VARCHAR(64)
);

CREATE TABLE group_chat_message (
    message_id INT AUTO_INCREMENT PRIMARY KEY,
    author_id INT NOT NULL,
    send_time DATETIME NOT NULL,
    content VARCHAR(2048),
    chat_id INT NOT NULL,
    CONSTRAINT fk_message_author
        FOREIGN KEY(author_id) REFERENCES profile(profile_id),
    CONSTRAINT fk_message_group_chat
        FOREIGN KEY(chat_id) REFERENCES group_chat(group_chat_id)
);

CREATE TABLE group_chat_member (
    member_id INT AUTO_INCREMENT PRIMARY KEY,
    profile_id INT NOT NULL,
    group_chat_id INT NOT NULL,
    CONSTRAINT fk_profile
        FOREIGN KEY(profile_id) REFERENCES profile(profile_id),
    CONSTRAINT fk_member_group_chat
        FOREIGN KEY(group_chat_id) REFERENCES group_chat(group_chat_id)
);
