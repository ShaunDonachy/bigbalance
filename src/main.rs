extern crate memmap;

use std::fs::OpenOptions;

use memmap::MmapMut;

fn init_balancesheet() {
    const PATH: &str = "BALANCE";
    let file = OpenOptions::new().read(true).write(true).create(true).open(&PATH).unwrap();
    // 25 million users 4 bytes per balance
    file.set_len(25_000_000 * 4).unwrap();

    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };

    for i in 0..25_000_000 {
        mmap[i * 4 + 0] = 5;
        mmap[i * 4 + 1] = 0;
        mmap[i * 4 + 2] = 0;
        mmap[i * 4 + 3] = 0;
    }
}

fn main() {
    init_balancesheet();
    //im editing main
}
