-- Your SQL goes her
-- The date and time when a class start in a DAY
CREATE TABLE class_time(
    day TINYINT NOT NULL,
    id_class INT NOT NULL,
    time_out DATETIME NOT NULL,
    time_in DATETIME NOT NULL,
    PRIMARY KEY(id_class, day),
    FOREIGN KEY (id_class) REFERENCES class(id_class)
);
