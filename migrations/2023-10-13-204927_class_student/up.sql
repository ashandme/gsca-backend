-- Your SQL goes here
CREATE TABLE class_student(
    id_class INT UNSIGNED REFERENCES class(id) NOT NULL,
    id_student INT UNSIGNED REFERENCES student(id) NOT NULL,
    PRIMARY KEY(id_class, id_student)
);
