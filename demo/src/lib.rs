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
mod r7_armstrong_numbers;
mod r8_grains;
mod r9_high_scores;
mod r10_difference_of_squares;
mod r11_leap;
mod r12_nth_prime;
mod r13_raindrop;
mod r14_sum_of_multiples;
mod r15_series;
mod fileter_any_windows;

pub struct Payload {
    pub features: Vec<String>,
}

pub struct Root {
    pub code: i64,
    pub success: bool,
    pub payload: Payload,
}

