#![feature(proc_macro_hygiene, decl_macro)]
#![feature(never_type)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

#[macro_use] 
extern crate rocket_contrib;
extern crate itertools;
extern crate rcir;

mod schema;

use diesel::SqliteConnection;
use rocket::Rocket;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Form, FromRequest, Request};
use rocket_contrib::{templates::Template, json::Json};

use schema::{Ballot, Item, Vote, NewUser};

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

// DB Pooling setup, if needed:
//
// An alias to te type for a pool of Diesel SQLite connections.
// type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
//
// The URL to the database, set via the 'DATABASE_URL' environment variable.
// static DATABASE_URL: &'static str = env!("DATABASE_URL");
//
/// Initializes a database pool.
// fn init_pool() -> SqlitePool {
//    let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_URL);
//    Pool::new(manager>).expect("db pool")
//}


#[derive(Debug, Serialize)]
struct Context {
    winner: Option<Item>,
    second: Option<Item>,
    items: Vec<(Item, Option<i32>)>,
}

impl Context {
    pub fn new(conn: &DbConn) -> Context {
        Context { 
            winner: Vote::run_election(conn),
            second: None,
            items: Vec::new(), // Not used if not logged in
        }
    }

    pub fn for_user(user: Auth, conn: &DbConn) -> Context {
        let winner = Vote::run_election(conn);
        let second = Vote::run_second_election(conn, &winner);
        Context {
            winner,
            second,
            items: Item::for_user(user.0, conn),
        }
    }
}

#[derive(Debug)]
struct Auth(i32);

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, !> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Auth(id))
            .or_forward(())
    }
}

// Routes
// 
// Standard practice is to put these in src/routes.rs but since this is dead
// simple, we're putting them in here. This makes rocket::ignite().mount simpler
// as well.
//
#[post("/login", data = "<input>")]
fn login(mut cookies: Cookies, input: Form<NewUser>, conn: DbConn) -> Template {
    let user = input.into_inner();
    if user.username.is_empty() {
        index(conn)
    } else {
        let u = user.login(&conn);
        cookies.add_private(Cookie::new("user_id", u.id.to_string()));
        votes(Auth(u.id), conn)
    }
}

#[post("/vote", data = "<ballot>")]
fn vote(ballot: Json<Ballot>, user: Auth, conn: DbConn) -> &'static str {
    Vote::save_ballot(user.0, ballot.into_inner(), &conn);
    "voted"
}

#[get("/")]
fn votes(user: Auth, conn: DbConn) -> Template {
    Template::render("vote", Context::for_user(user, &conn))
}

#[get("/", rank = 2)]
fn index(conn: DbConn) -> Template {
    Template::render("index", Context::new(&conn))
}

#[head("/")]
fn index_head(conn: DbConn) -> Template {
    index(conn)
}


// Fire up the Rocket Server!
// 
fn rocket() -> (Rocket, Option<DbConn>) {
    let rocket = rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes![index, index_head, login, votes, vote])
        .attach(Template::fairing());

    let conn = match cfg!(test) {
        true => DbConn::get_one(&rocket),
        false => None,
    };

    (rocket, conn)
}

/* 
#[launch]
fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        //.mount("/", StaticFiles::from(crate_relative!("/static")))
        .mount("/", routes![index])
        // .mount("/todo", routes![new, toggle, delete])
        .attach(Template::fairing())
}
*/


fn main() {
    rocket().0
    // If using managed pool, this next config line. For details see:
    //      https://rocket.rs/v0.3/guide/state/ -- don't forget Cnxn Guard.
    // .manage(init_pool())
    .launch();
}
