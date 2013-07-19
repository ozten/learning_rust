use std::hashmap::HashMap;

fn main() {
    // HashMap play - One type of Json is Object which is a
    // type over HashMap<~str, Json>
    let mut h: HashMap<~str, ~str> = HashMap::new();
    h.insert(~"foo", ~"bar");
    h.get(& ~"foo");

    let mut a:~[~str] = ~[];
    a.push(~"Hello");
    a.push(~" World");
    println(a.concat());
}
