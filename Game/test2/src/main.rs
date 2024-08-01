use std::{cell::OnceCell, iter::Once, sync::OnceLock};

use lazy_static::lazy_static;

lazy_static! {
    static ref ENEMY: String = {
        "Monster".to_string()
    };
}

fn main() {
    print!("Hacked by Jon2D")
}
