-- Your SQL goes her
CREATE TABLE user (
    id_user INT NOT NULL,
    dni INT NOT NULL,
    secret VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    rol VARCHAR(15) NOT NULL,
    alias VARCHAR(20) NOT NULL,
    PRIMARY KEY(id_user),
    UNIQUE KEY(dni)
);
