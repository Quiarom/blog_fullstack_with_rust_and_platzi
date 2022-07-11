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

    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_posts = NewPost {
        title: "Mi primer blog",
        body: "Es mi primera oración",
        slug: "primer-post",
    };

    
    diesel::insert_into(posts::table).values(&new_posts).get_result::<Post>(&connection_db).expect("Error Insert Tables");
    // let post: Post = diesel::insert_into(posts::table).values(&new_posts).get_result(&connection_db).expect("Error Insert Tables");

    // Select * from posts limit 1CfxDPafaHJuw72
    let posts_result = posts.limit(1).load::<Post>(&connection_db).expect("Error ejecute query");

    for post in posts_result {
       println!("{}", post.title); 
    }
}
