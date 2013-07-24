extern mod extra;
extern mod sqlite;

use extra::comm::DuplexStream;

use sqlite::open;
use sqlite::database::Database;
use sqlite::types::*;

static create_repo_meta:&'static str =
    "CREATE TABLE IF NOT EXISTS repo_meta (   \
        id INTEGER PRIMARY KEY AUTOINCREMENT, \
        url TEXT NOT NULL, next_url TEXT,     \
        json TEXT, created DATE               \
    );";
//TODO: When using fmt! we cant have static strings?
static insert_repo_meta:&'static str =
    "INSERT INTO repo_meta                    \
        (id, url, next_url, json, created)    \
    VALUES                                    \
        (null, '%s', null, null, datetime('now'));";

static update_repo_meta:&'static str =
    "UPDATE repo_meta                         \
     SET next_url = '%s', json = '%s'         \
     WHERE url = '%s';";

static next_repo_url:&'static str =
    "SELECT url FROM repo_meta                \
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
                // TODO return api.github.com url, fixup
                // in repo fetcher side
                let next_url = ~"https://api.github.com/repositories";
                insert_next_url_repo_meta(database, next_url);
                return next_url;
            } else {
                return stmt.get_text(0);
            }
        },
        Err(ref e) => {
            fail!("%?", e);
        }
    };
}

/*  assert!(sth.bind_param(1, &Integer(2)) == SQLITE_OK);
    assert!(sth.step() == SQLITE_ROW); */
fn insert_next_url_repo_meta(database: &Database, next_url: &str) {
    debug!("Going for it");
    debug!(insert_repo_meta);
    let sql = fmt!(
    "INSERT INTO repo_meta                    \
        (id, url, next_url, json, created)    \
     VALUES                                   \
        (null, '%s', null, null, datetime('now'));", next_url);
    match database.exec(sql) {
        Ok(true) => {
            debug!("finished with insert");
        },
        Ok(false) => {
            debug!("failed with insert");
        },
        Err(e) => {
            fail!(fmt!("insert_db prep stmt failed! %?", e));
        }
    }

    //println(fmt!("INSERT %?", database.exec("INSERT INTO test (foo) VALUES (\"hamster dance\");")));
}

fn update_json_repo_meta(database: &Database, url: &str,
                    json: &str, next_url: &str) {
    // ' are crashing sqlite query, so we HTML encode them. lame.
    let escJson = json.replace("'", "&#39;").replace("\n", " ");
    let sql = fmt!(
        "UPDATE repo_meta                         \
         SET next_url = '%s', json = '%s'         \
         WHERE url = '%s';", next_url, escJson, url);
    println(sql);
    match database.exec(sql) {
        Ok(true) => {
            debug!("finished with update");
        },
        Ok(false) => {
            debug!("failed with update");
        }
        Err(e) => {
            fail!(fmt!("update_db prep stmt failed! %?", e));
        }
    }
}

// Task
fn init_db(chan:&DuplexStream<~str, ~str>) -> () {
    debug!("Database starting");
    let database = match open("github.sqlite") {
        Ok(db) => db,
        Err(e) => {
            fail!("Unable to open sqlite database")
        }
    };
    setup(&database);
    chan.send(next_url(&database));

    debug!("I'll wait till I get a current_url");
    let current_url:&str = chan.recv();

    debug!("I'll wait till I get the json");
    let json:&str = chan.recv();

    debug!("I'll wait till I get a next_url");
    let next_url:&str = chan.recv();
    debug!(next_url);

    //debug!(json);
    debug!("Okay thenz");
    update_json_repo_meta(&database, current_url, json, next_url);
    debug!("Okay, let's insert the next url");
    insert_next_url_repo_meta(&database, next_url);
    debug!("Phew... All done.");
    //loop {
        // wait for read or writes to the database
        // execute them
    //}
}