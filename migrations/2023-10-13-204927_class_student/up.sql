-- Your SQL goes here
CREATE TABLE class_student(
    id_class INT UNSIGNED NOT NULL REFERENCES class(id),
    id_student INT UNSIGNED NOT NULL REFERENCES student(id),
    PRIMARY KEY(id_class, id_student)
);
