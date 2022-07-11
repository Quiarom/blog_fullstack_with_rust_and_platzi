#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;


use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Not Found db_url Variable");

    let connection_db = PgConnection::establish(&db_url).expect("Nou Possible Connection with Database");

    use self::models::Post;
    use self::schema::posts::dsl::*;
    let posts_result = posts.load::<Post>(&connection_db).expect("Error ejecute query");

    for post in posts_result {
       println!("{}", post.title); 
    }
}
