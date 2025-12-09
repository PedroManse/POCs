//! [Send] = Pode enviar `T`
//!
//! [Sync] = Pode enviar `&T`

use std::cell::Cell;
use std::rc::Rc;

// Não compila
pub fn not_send(x: Rc<i32>) {
    std::thread::spawn(move || {
        let k = x;
    });
}

// Não compila
pub fn so_its_not_sync(x: Rc<i32>) {
    std::thread::spawn(|| {
        let k = &x;
    });
}

// Compila
pub fn is_send(x: Cell<i32>) {
    std::thread::spawn(move || {
        let k = x;
    });
}

// Não compila
pub fn but_not_sync(x: Cell<i32>) {
    std::thread::spawn(|| {
        let k = &x;
        k.set(5);
    });
}

// trait Sync<T: Send>
// Então não existe T que: ?Send + Sync

// Compila?
pub fn why_is_this_sync(x: &mut i32) {
    std::thread::scope(|s| {
        s.spawn(|| {
            let k = &x;
        });
    });
}
