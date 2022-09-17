mod a1_file;
mod demo1;
mod r1_hello;
mod r2_reverse;
mod r3_after;
mod r4_clock;
mod r5_anagram;
mod r5_anagram2;
mod r6_space_age;
mod r6_space_age2;

pub struct Payload {
    pub features: Vec<String>,
}

pub struct Root {
    pub code: i64,
    pub success: bool,
    pub payload: Payload,
}
