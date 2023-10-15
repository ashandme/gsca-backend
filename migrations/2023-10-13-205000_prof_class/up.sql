-- Your SQL goes here

CREATE TABLE prof_class(
    id_class INT UNSIGNED REFERENCES class(id),
    id_user INT UNSIGNED REFERENCES user(id),
    PRIMARY KEY(id_user, id_class)
);
