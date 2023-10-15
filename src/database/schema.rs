// @generated automatically by Diesel CLI.

diesel::table! {
    class (id) {
        id -> Unsigned<Integer>,
        #[max_length = 20]
        area -> Nullable<Varchar>,
        #[max_length = 20]
        subject -> Varchar,
        #[max_length = 3]
        year_div -> Varchar,
        date_start -> Date,
        date_end -> Date,
    }
}

diesel::table! {
    class_day (id) {
        id -> Unsigned<Integer>,
        day -> Tinyint,
        id_class -> Nullable<Unsigned<Integer>>,
        time_out -> Time,
        time_in -> Time,
    }
}

diesel::table! {
    class_student (id_class, id_student) {
        id_class -> Unsigned<Integer>,
        id_student -> Unsigned<Integer>,
    }
}

diesel::table! {
    prof_class (id_user, id_class) {
        id_class -> Unsigned<Integer>,
        id_user -> Unsigned<Integer>,
    }
}

diesel::table! {
    reg (id) {
        id -> Unsigned<Integer>,
        id_student -> Nullable<Unsigned<Integer>>,
        time_in -> Date,
        time_out -> Date,
        class_day -> Nullable<Unsigned<Integer>>,
    }
}

diesel::table! {
    student (id) {
        id -> Unsigned<Integer>,
        id_fingerprint -> Nullable<Unsigned<Integer>>,
        dni -> Integer,
        #[max_length = 20]
        name -> Varchar,
        #[max_length = 20]
        surname -> Varchar,
    }
}

diesel::table! {
    user (id) {
        id -> Unsigned<Integer>,
        dni -> Integer,
        #[max_length = 255]
        secret -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 16]
        rol -> Varchar,
        #[max_length = 32]
        alias -> Varchar,
    }
}

diesel::joinable!(class_day -> class (id_class));
diesel::joinable!(class_student -> class (id_class));
diesel::joinable!(class_student -> student (id_student));
diesel::joinable!(prof_class -> class (id_class));
diesel::joinable!(prof_class -> user (id_user));
diesel::joinable!(reg -> class_day (class_day));
diesel::joinable!(reg -> student (id_student));

diesel::allow_tables_to_appear_in_same_query!(
    class,
    class_day,
    class_student,
    prof_class,
    reg,
    student,
    user,
);
