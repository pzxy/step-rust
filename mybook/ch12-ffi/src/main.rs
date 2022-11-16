// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

use std::{ptr, result};

// 这里需要绝对路径，OUT_DIR 是rust执行时候，当前路径，这个不需要自己设置。
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

type Result = result::Result<Vec<String>, String>;
fn match_string(pattern: &str, subject: &str) -> Result {
    unsafe {
        let mut err_number = 0;
        let mut err_offset = 0;
        let re = pcre2_compile_8(
            pattern.as_ptr(), /* the pattern */
            pattern.len(),
            0,               /* default options */
            &mut err_number, /* for error number */
            &mut err_offset, /* for error offset */
            ptr::null_mut(), /* use default compile context */
        );
        if re == ptr::null_mut() {
            return Err(format!(
                "re:{:?},err_number:{:?},err_offset:{:?}",
                re, err_number, err_offset
            ));
        }
        let match_data = pcre2_match_data_create_from_pattern_8(re, ptr::null_mut());
        //释放空间
        let rc = pcre2_match_8(
            re,
            subject.as_ptr() as PCRE2_SPTR8,
            subject.len(),
            0,
            0,
            match_data,
            ptr::null_mut(),
        );
        if rc < 0 {
            pcre2_match_data_free_8(match_data); /* Release memory used for the match */
            pcre2_code_free_8(re); /*   data and the compiled pattern. */
            return match rc {
                PCRE2_ERROR_NOMATCH => Err("No match".to_string()),
                _ => Err(format!("Matching error {:?}", rc)),
            };
        }
        if rc == 0 {
            println!("output vector was not big enough for all the captured substrings");
        }
        let mut ret: Vec<String> = vec![];
        let mut idx1 = 1;
        let mut idx2 = 0;
        let mut i = 0;
        let output_vector = pcre2_get_ovector_pointer_8(match_data);
        while idx1 != idx2 {
            idx1 = *output_vector.offset(i);
            idx2 = *output_vector.offset(i + 1);
            println!("idx:{},idx2:{}", idx1, idx2);
            if idx1 > idx2 {
                pcre2_match_data_free_8(match_data); /* Release memory used for the match */
                pcre2_code_free_8(re); /*   data and the compiled pattern. */
                return Err(format!(
                    "from end to start the match was: {:?}",
                    subject[idx2..idx1].to_string()
                ));
            }
            if idx1 == idx2 {
                break;
            }
            if idx2 > subject.len() {
                break;
            }
            ret.push(subject[idx1..idx2].to_string());
            i += 2;
        }
        return Ok(ret);
    }
}

fn main() {
    let pattern = r"\d{4}([^0-9^\s]{3,11})\S";
    let src= "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let ret = match_string(pattern, src);
    match ret {
        Ok(v) => {
            if v.len() < 2 {
                panic!("match fail")
            }
            if let Some(msg) = v.get(1) {
                println!("{}", msg);
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
