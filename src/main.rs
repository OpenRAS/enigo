use std::{thread::sleep, time::Duration};

use enigo::{Enigo, Key, KeyboardControllable};

fn main() {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Meta);
    enigo.key_click(Key::Layout('a'));
    enigo.key_up(Key::Meta);
    sleep(Duration::from_secs(1));
    enigo.key_sequence_parse("{+SHIFT}a{-SHIFT}");
    sleep(Duration::from_secs(1));
}
