extern mod sqlite;

use sqlite::open;
use sqlite::database::Database;
use sqlite::types::*;

static create_repo_meta:&'static str =
    "CREATE TABLE IF NOT EXISTS repo_meta (   \
        id INTEGER PRIMARY KEY AUTOINCREMENT, \
        url TEXT NOT NULL, next_url TEXT,     \
        json TEXT, created DATE               \
    );";

static insert_repo_meta:&'static str =
    "INSERT INTO repo_meta                    \
        (id, url, next_url, json, created)    \
    VALUES
        (null, \"https://github.com/repositoryies\", null, null, datetime('now'));";

static next_repo_url:&'static str =
    "SELECT next_url FROM repo_meta           \
     WHERE next_url IS NULL AND json IS NULL  \
     ORDER BY created DESC LIMIT 1;";

fn setup(database: &Database) {
    let res:SqliteResult<bool> = database.exec(create_repo_meta);
    match res {
        Ok(true) => { debug!("Ensured repo_meta exists"); },
        Ok(false) => { debug!("Failed creating repo_meta"); },
        Err(e) => { fail!(fmt!("%?", e)); }
    }
}

fn next_url(database: &Database) -> ~str {
        let cur = database.prepare(next_repo_url, &None);
        match cur {
            Ok(ref stmt) => {
                if (stmt.step() == SQLITE_DONE) {
                    return ~"https://localhost:8002/repositories";
                } else {
                    return stmt.get_text(0);
                }
            },
            Err(ref e) => {
                fail!("%?", e);
            }
        };
}

/*
database.prepare
let sth = checked_prepare(database, "SELECT id FROM test WHERE id > ? AND id < ?");
    assert!(sth.bind_param(1, &Integer(2)) == SQLITE_OK);
    assert!(sth.bind_param(2, &Integer(4)) == SQLITE_OK);

    assert!(sth.step() == SQLITE_ROW);
    assert!(sth.get_num(0) as int == 3);
    */
fn insert_db() {
    //println(fmt!("INSERT %?", database.exec("INSERT INTO test (foo) VALUES (\"hamster dance\");")));
}

// Task
fn init_db(start_channel:&Chan<~str>) -> () {
    debug!("Database starting");
    let database = match open("github.sqlite") {
        Ok(db) => db,
        Err(e) => {
            fail!("Unable to open sqlite database")
        }
    };
    setup(&database);
    start_channel.send(

        next_url(
            &database));
    //loop {
        // wait for read or writes to the database
        // execute them
    //}
}