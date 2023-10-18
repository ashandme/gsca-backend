-- Your SQL goes here

CREATE TABLE prof_class(
    id_class INT UNSIGNED REFERENCES class(id) NOT NULL,
    id_user INT UNSIGNED REFERENCES user(id) NOT NULL,
    PRIMARY KEY(id_user, id_class)
);
