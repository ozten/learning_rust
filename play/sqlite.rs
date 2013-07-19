// rustc -L ../libs/rustsqlite sqlite.rs

extern mod sqlite;
extern mod types;

use sqlite::*;
use types::*;

error!("error here")

fn main() {
    match open(~"play.sqlite") {
        Ok(database) => {
            // TODO Check ResultCode enum
            let res:types::SqliteResult<bool> = database.exec("CREATE TABLE IF NOT EXISTS test (foo TEXT);");
            match res {
                Ok(true) => {"Boo Ya";},
                Ok(false) => {"Hmmm"; },
                Err(e) => {println(fmt!("%?", e));}
            }
            println(fmt!("CREATE %?", res));
            println(fmt!("INSERT %?", database.exec("INSERT INTO test (foo) VALUES (\"hamster dance\");")));
            let cur = database.prepare("SELECT * FROM test", &None);
            match cur {
                Ok(ref stmt) => {
                    debug!("DEBUG Statement: %?", stmt);
                    stmt.step();
                    debug!("DEBUG Statment: %?", stmt.get_text(0));
                },
                Err(ref e) => {println(fmt!("%?", e));}
            }
            println(fmt!("SELECT %?", cur));
        },
        Err(e) => {
            fail!("Unable to open sqlite database")
        }
    }
}