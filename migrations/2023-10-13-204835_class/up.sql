-- Your SQL goes here
CREATE TABLE class(
    id_class INT NOT NULL,
    area VARCHAR(20) NOT NULL,
    subject VARCHAR(20) NOT NULL,
    year_div VARCHAR(3) NOT NULL,
    date_start DATE NOT NULL,
    date_end DATE NOT NULL,
    PRIMARY KEY(id_class)
);
