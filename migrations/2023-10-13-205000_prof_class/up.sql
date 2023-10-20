-- Your SQL goes here

CREATE TABLE prof_class(
    id_class INT UNSIGNED NOT NULL REFERENCES class(id),
    id_user INT UNSIGNED NOT NULL REFERENCES user(id),
    PRIMARY KEY(id_user, id_class)
);
