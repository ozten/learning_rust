extern mod sqlite;

use sqlite::*;

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


fn setup() {
    debug!("Database starting");
    match open("github.sqlite") {
        Ok(database) => {
            // TODO Check ResultCode enum
            let res:types::SqliteResult<bool> =
                database.exec(create_repo_meta);
            match res {
                Ok(true) => { debug!("Ensured repo_meta exists"); },
                Ok(false) => { debug!("Failed creating repo_meta"); },
                Err(e) => { fail!(fmt!("%?", e)); }
            }
            /*
            println(fmt!("CREATE %?", res));
            println(fmt!("INSERT %?", database.exec("INSERT INTO test (foo) VALUES (\"hamster dance\");")));
            let cur = database.prepare("SELECT * FROM test", &None);
            match cur {
                Ok(ref stmt) => {
                    debug!(stmt);
                    stmt.step();
                    debug!(stmt.get_text(0));
                },
                Err(ref e) => {println(fmt!("%?", e));}
            }
            println(fmt!("SELECT %?", cur));
            */
        },
        Err(e) => {
            fail!("Unable to open sqlite database")
        }
    }
}

// Task
fn init_db(start_channel:&Chan<~str>) {
    println("Calling setup");
    setup();
    start_channel.send(~"https://localhost:8002/repositories");
    //loop {
        // wait for read or writes to the database
        // execute them
    //}
}