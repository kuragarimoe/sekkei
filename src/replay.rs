struct Frame {
    time: i32,
    x: i32,
    y: i32,
    keys: i32
}

pub enum Keys {
    M1 = 1 << 0,
    M2 = 1 << 1,
    K1 = 1 << 2,
    K2 = 1 << 3
}