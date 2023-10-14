use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use diesel::mysql::MysqlConnection;
mod database::models;
type DbPool = Pool<ConnectionManager<MysqlConnection>>;

async fn add_student(
    student_info: web::Json<Student>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let student = student_info.into_inner();
    let conn = pool.get().expect("Error al conectar a la base de datos");

    // Inserta el estudiante en la base de datos
    let new_student = diesel::insert_into(students::table)
        .values(&Student {
            id: None,
            nombre: student.nombre,
            edad: student.edad,
        })
        .execute(&conn);

    match new_student {
        Ok(_) => HttpResponse::Ok().body("Estudiante agregado correctamente"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error al agregar estudiante: {}", e)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Configura la conexión a la base de datos
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL no está configurada en .env");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::new(manager).expect("Error al crear la pool de conexiones");

    // Configura y ejecuta el servidor Actix
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/student/add", web::post().to(add_student))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
