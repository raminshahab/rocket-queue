#[macro_use] 
extern crate rocket; // This is required if you use Rocket macros like #[launch]
#[macro_use] 
extern crate diesel;

use diesel::prelude::*;
use rocket_sync_db_pools::database;
use rocket::serde::json::Json;
use diesel::PgConnection;

mod schema;
mod models;

use crate::schema::users::dsl::{users, name, email};
use crate::models::User;
use diesel::r2d2::{ConnectionManager, Pool};

type DbPool = Pool<ConnectionManager<PgConnection>>;
type DbConn = rocket_sync_db_pools::Connection<DbPool>;

#[database("postgres_db")]
pub struct DbConn(Pool<ConnectionManager<PgConnection>>); // Store the pool itself

#[get("/user/<id>")]
async fn get_user(id: i32, conn: DbConn) -> Option<Json<User>> {
    use crate::schema::users::dsl::*;
    
    // Run the query with a connection from the pool
    let user = conn.run(move |c| {
        users.filter(id.eq(&id)).first::<User>(c)
    }).await.ok();
    
    user.map(Json)
}

#[post("/user", data = "<user>")]
async fn create_user(user: Json<User>, conn: DbConn) -> Json<User> {
    conn.run(move |c| {
        diesel::insert_into(users)
            .values(&*user)
            .get_result(c)
    }).await.unwrap()
}

#[put("/user/<id>", data = "<updated_user>")]
async fn update_user(id: i32, updated_user: Json<User>, conn: DbConn) -> Option<Json<User>> {
    conn.run(move |c| {
        diesel::update(users.filter(id.eq(&id)))
            .set((name.eq(&updated_user.name), email.eq(&updated_user.email)))
            .get_result(c)
    }).await.ok()
    .map(Json)
}

#[delete("/user/<id>")]
async fn delete_user(id: i32, conn: DbConn) -> Option<Json<String>> {
    conn.run(move |c| {
        diesel::delete(users.filter(id.eq(&id)))
            .execute(c)
    }).await.ok()
    .map(|_| Json(format!("User with ID {} deleted.", id)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
}
