
fn main() {
    println(fmt!("%?", [
        "foo",
        "bar"
        ].connect("\u000D\u000A").append("foo")));

    let a = [
        ("one", 1),
        ("two", 2),
        ("three", 3)
    ];

    for a.iter().advance |tup:&(&'static str, int)| {
        fn mkheader(key: &str, value: &int) -> ~str {
            ~"bar"
        }
        //let headers = tup.map(mkheader);
        //println(fmt!("%?", headers));
    }
}