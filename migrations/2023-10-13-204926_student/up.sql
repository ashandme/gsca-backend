-- Your SQL goes here
CREATE TABLE student(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    id_fingerprint INT UNSIGNED,
    dni INT NOT NULL,
    name VARCHAR(20) NOT NULL,
    surname VARCHAR(20) NOT NULL,
    UNIQUE KEY(dni)
);
