// rustc -L ../libs/rustsqlite sqlite.rs

extern mod sqlite;
extern mod types;

use sqlite::*;
use sqlite::types::*;

error!("error here")

fn main() {
    match open(~"play.sqlite") {
        Ok(database) => {
            // TODO Check ResultCode enum
            let res:types::SqliteResult<bool> = database.exec(
                "CREATE TABLE IF NOT EXISTS test (foo TEXT);");
            match res {
                Ok(true) => {"Boo Ya";},
                Ok(false) => {"Hmmm"; },
                Err(e) => {println(fmt!("%?", e));}
            }
            println(fmt!("CREATE %?", res));
            println(fmt!("INSERT %?", database.exec(
                fmt!("INSERT INTO test (foo) VALUES (\"%s\");", "hamster dance"))));
            let cur = database.prepare(
                fmt!("SELECT foo FROM test WHERE foo = \"%s\"",
                     "hamster dance"), &None);
            match cur {
                Ok(ref stmt) => {
                    stmt.step();
                    debug!("Getting output");
                    debug!(stmt.get_text(0));
                    debug!("Resetting");
                },
                Err(ref e) => {println(fmt!("%?", e));}
            }
            println(fmt!("SELECT %?", cur));
        },
        Err(e) => {
            fail!("Unable to open sqlite database")
        }
    }
    println("Yay");
}