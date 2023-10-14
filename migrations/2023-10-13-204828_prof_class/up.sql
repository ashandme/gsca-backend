-- Your SQL goes here

CREATE TABLE prof_class(
    id_class INT NOT NULL,
    id_user INT NOT NULL,
    PRIMARY KEY(id_class, id_user),
    FOREIGN KEY (id_class) REFERENCES class(id_class),
    FOREIGN KEY (id_user) REFERENCES user(id_user)
);
