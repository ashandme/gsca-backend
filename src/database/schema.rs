// @generated automatically by Diesel CLI.

diesel::table! {
    class (id_class) {
        id_class -> Integer,
        #[max_length = 20]
        area -> Varchar,
        #[max_length = 20]
        subject -> Varchar,
        #[max_length = 3]
        year_div -> Varchar,
        date_start -> Date,
        date_end -> Date,
    }
}

diesel::table! {
    class_student (id_class) {
        id_class -> Integer,
        id_student -> Integer,
    }
}

diesel::table! {
    class_time (id_class) {
        id_class -> Integer,
        time_out -> Datetime,
        time_in -> Datetime,
    }
}

diesel::table! {
    prof_class (id_class) {
        id_class -> Integer,
        id_user -> Integer,
    }
}

diesel::table! {
    reg (id_reg) {
        id_reg -> Integer,
        id_student -> Integer,
        time_in -> Date,
        time_out -> Date,
        id_class -> Integer,
    }
}

diesel::table! {
    student (id_student) {
        id_student -> Integer,
        id_fingerprint -> Nullable<Integer>,
        dni -> Integer,
        #[max_length = 20]
        name -> Varchar,
        #[max_length = 20]
        surname -> Varchar,
    }
}

diesel::table! {
    user (id_user) {
        id_user -> Integer,
        dni -> Integer,
        #[max_length = 255]
        secret -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 15]
        rol -> Varchar,
        #[max_length = 20]
        alias -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    class,
    class_student,
    class_time,
    prof_class,
    reg,
    student,
    user,
);
