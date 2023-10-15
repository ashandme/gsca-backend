-- Your SQL goes here
CREATE TABLE class(
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    area VARCHAR(20),
    subject VARCHAR(20) NOT NULL,
    year_div VARCHAR(3) NOT NULL,
    date_start DATE NOT NULL,
    date_end DATE NOT NULL
);
