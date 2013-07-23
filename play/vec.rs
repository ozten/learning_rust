
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

    let b = [
        ~[~"one", ~"uno"],
        ~[~"two", ~"dos"],
        ~[~"three", ~"tres"]
    ];
    fn mkheaderz(el:&~[~str]) -> ~str {
        match el {
            [eng, span] => copy (eng + ": " + span),
            _ => {fail!("ouch");}
        }
    }

    //let c:~[~str] =
    b.flat_map(mkheaderz);
}