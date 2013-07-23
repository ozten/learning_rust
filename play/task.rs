// rustc task.rs
// Works on Rust 0.7

extern mod extra;
extern mod std;

use std::comm::select2i;
use std::task;

use extra::comm::DuplexStream;
use extra::timer::sleep;
use extra::uv;

fn main() {
    let (receiver, sender): (Port<~str>, Chan<~str>) = stream();
    do spawn {
        loop {
            sender.send(~"Hey there");
            sleep(&uv::global_loop::get(), 3000);

        }
    };

    let (receiver2, sender2): (Port<~str>, Chan<~str>) = stream();

    //:(DuplexStream<~str, ~str>, DuplexStream<~str, ~str>)
    let (from_child, to_child) = DuplexStream();

    do spawn {
        loop {
            sleep(&uv::global_loop::get(), 1000);
            sender2.send(~"Second child");
            if (to_child.peek()) {
                println(fmt!("%?", to_child.recv()));
            }
            to_child.send(~"Yo, yo");
        }
    };

    loop {
        if (receiver.peek()) {
            println(fmt!("%?", receiver.try_recv()));
        }
        if (receiver2.peek()) {
            println(fmt!("%?", receiver2.try_recv()));
        }
        if (from_child.peek()) {
                println(fmt!("%?", from_child.recv()));
            }
        from_child.send(~"Hambone, dinner");
    }
}