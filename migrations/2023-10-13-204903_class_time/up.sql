-- Your SQL goes her
-- The date and time when a class start in a DAY
CREATE TABLE class_day(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    day TINYINT NOT NULL,
    id_class INT UNSIGNED REFERENCES class(id) NOT NULL,
    time_out TIME NOT NULL,
    time_in TIME NOT NULL
);
