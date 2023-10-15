-- Your SQL goes here

CREATE TABLE prof_class(
    id_profe INT NOT NULL,
    id_class INT NOT NULL,
    id_user INT NOT NULL,
    PRIMARY KEY(id_profe),
    FOREIGN KEY (id_class) REFERENCES class(id_class),
    FOREIGN KEY (id_user) REFERENCES user(id_user)
);
