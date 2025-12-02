use std::cell::Cell;
use std::rc::Rc;

/// [Sync] = Pode enviar &T
/// [Send] = Pode enviar T

fn not_send(x: Rc<i32>) {
    std::thread::spawn(move || {
        let k = x;
    });
}

fn so_its_not_sync(x: Rc<i32>) {
    std::thread::spawn(|| {
        let k = &x;
    });
}

fn is_send(x: Cell<i32>) {
    std::thread::spawn(move || {
        let k = x;
    });
}

fn but_not_sync(x: Cell<i32>) {
    std::thread::spawn(|| {
        let k = &x;
        k.set(5);
    });
}

//fn not_send(x: ?) {}
//fn but_is_sync(x: ?) {}

fn why_is_this_sync(x: &mut i32) {
    std::thread::scope(|s| {
        s.spawn(|| {
            let k = &x;
        });
    });
}
