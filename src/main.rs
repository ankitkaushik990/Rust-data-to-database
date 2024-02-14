#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    
    if let Ok(url) = env::var("DATABASE_URL") {
        let conn = PgConnection::establish(&url)
            .expect("could not connect to database");

            insert_todo(&conn, "todo01");
            insert_todo(&conn, "todo02");
            insert_todo(&conn, "todo03");
            insert_todo(&conn, "todo04");

        query_todos(&conn);
        
    } else {
        println!("No DATABASE_URL set")
    }
}

use diesel::prelude::*;

#[derive(Debug, Queryable)]
struct Todo {
    id: i32,
    name: String,
}


pub mod schema;

use schema::todos;

fn query_todos(conn: &PgConnection) {
    let rows = todos::table
        .limit(10)
        .load::<Todo>(conn)
        .expect("could not load todos");
    for row in rows {
        println!("{:?}", row);
    }
}


#[derive(Debug, Insertable, QueryableByName)]
#[table_name="todos"]
struct NewTodo<'a> {
    name: &'a str,
}


fn insert_todo<'a>(conn: &PgConnection, name: &'a str) -> Todo {
    let new_todo = NewTodo{
        name: name,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("error saving new todo")
}


