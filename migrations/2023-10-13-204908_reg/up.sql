-- Your SQL goes here
CREATE TABLE reg(
    id_reg INT NOT NULL,
    id_student INT NOT NULL,
    time_in DATE NOT NULL,
    time_out DATE NOT NULL,
    id_class INT NOT NULL,
    PRIMARY KEY(id_reg),
    FOREIGN KEY (id_student) REFERENCES student(id_student),
    FOREIGN KEY (id_class) REFERENCES class(id_class)
);
