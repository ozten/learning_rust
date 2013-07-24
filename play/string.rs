// rustc string.rs && RUST_LOG=string=4 ./string

fn main() {
    let s = "<foo bar>";
    match s.find('<') {
        Some(start) => {
            println(fmt!("%?", start));
            match s.find('>') {
                Some(end) => {
                    println(s.slice(start + 1, end));
                },
                _ => { fail!("Missing >"); }
            }

        },
        _ => {fail!("Missing <"); }
    }
    println(s);
    println("So we've come this far haven't we?\nHaven't we???");
    println("So we've come this far haven't we?\nHaven't we???".replace("'", "\\'").replace("\n", ""));
}