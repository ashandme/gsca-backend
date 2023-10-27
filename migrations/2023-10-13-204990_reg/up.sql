-- Your SQL goes here
CREATE TABLE reg(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    id_student INT UNSIGNED NOT NULL REFERENCES student(id),
    caption VARCHAR(255),
    time_in DATETIME NOT NULL,
    time_out DATETIME NOT NULL,
    class_day INT UNSIGNED NOT NULL REFERENCES class_day(id)
);
