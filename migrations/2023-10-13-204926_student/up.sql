-- Your SQL goes here

CREATE TABLE student(
    id_student INT NOT NULL,
    id_fingerprint INT,
    dni INT NOT NULL,
    name VARCHAR(20) NOT NULL,
    surname VARCHAR(20) NOT NULL,
    PRIMARY KEY(id_student),
    UNIQUE KEY(dni)
);
