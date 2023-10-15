-- Your SQL goes here
CREATE TABLE reg(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    id_student INT UNSIGNED REFERENCES student(id),
    time_in DATE NOT NULL,
    time_out DATE NOT NULL,
    class_day INT UNSIGNED REFERENCES class_day(id)
);
