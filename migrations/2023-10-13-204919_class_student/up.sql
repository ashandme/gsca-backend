-- Your SQL goes here
CREATE TABLE class_student(
    id_class INT NOT NULL,
    id_student INT NOT NULL,
    PRIMARY KEY(id_class, id_student),
    FOREIGN KEY (id_class) REFERENCES class(id_class),
    FOREIGN KEY (id_student) REFERENCES class(id_student)
);
