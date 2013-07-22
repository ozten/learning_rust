
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


}