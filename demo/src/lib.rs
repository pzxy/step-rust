mod a1_file;
mod demo1;
mod r1_hello;
mod r2_reverse;
mod r3_after;
mod r4_clock;

pub struct Payload {
    pub features: Vec<String>,
}

pub struct Root {
    pub code: i64,
    pub success: bool,
    pub payload: Payload,
}
