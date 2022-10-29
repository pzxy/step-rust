#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::net::UdpSocket;
use std::{ptr, result};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_compression_decompression() {
        unsafe {
            // let mut context = pcre2_real_compile_context_8 { _unused: [] };
            // println!("1 context==>{:?}", &mut context);
            // pcre2_set_compile_extra_options_8(&mut context, 67108864);
            // pcre2_set_max_pattern_length_8(&mut context, 1024);
            // pcre2_set_newline_8(&mut context, PCRE2_NEWLINE_LF);
            //pub const PCRE2_NEWLINE_CR: u32 = 1;
            // pub const PCRE2_NEWLINE_LF: u32 = 2;
            // pub const PCRE2_NEWLINE_CRLF: u32 = 3;
            // pub const PCRE2_NEWLINE_ANY: u32 = 4;
            // pub const PCRE2_NEWLINE_ANYCRLF: u32 = 5;
            // pub const PCRE2_NEWLINE_NUL: u32 = 6;
            // let mut context = pcre2_real_compile_context_8 { _unused: [] };
            // println!("2 context==>{:?}", ccontext);
            // println!("3 context==>{:?}", ccontext);
            //pub const PCRE2_MATCH_INVALID_UTF: u32 = 67108864;
            // pub const PCRE2_EXTRA_ALLOW_SURROGATE_ESCAPES: u32 = 1;
            // pub const PCRE2_EXTRA_BAD_ESCAPE_IS_LITERAL: u32 = 2;
            // pub const PCRE2_EXTRA_MATCH_WORD: u32 = 4;
            // pub const PCRE2_EXTRA_MATCH_LINE: u32 = 8;
            // pub const PCRE2_EXTRA_ESCAPED_CR_IS_LF: u32 = 16;
            // pub const PCRE2_EXTRA_ALT_BSUX: u32 = 32;
            // pub const PCRE2_EXTRA_ALLOW_LOOKAROUND_BSK: u32 = 64;
            let mut err_number = 0;
            let mut err_offset = 0;
            let pattern = r"\d+";
            let src = String::from("hello 2033 world");

            // println!("4 context==>{:?}", ccontext);
            //pub const PCRE2_ANCHORED: u32 = 2147483648;
            // pub const PCRE2_NO_UTF_CHECK: u32 = 1073741824;
            // pub const PCRE2_ENDANCHORED: u32 = 536870912;
            // pub const PCRE2_ALLOW_EMPTY_CLASS: u32 = 1;
            // pub const PCRE2_ALT_BSUX: u32 = 2;
            // pub const PCRE2_AUTO_CALLOUT: u32 = 4;
            // pub const PCRE2_CASELESS: u32 = 8;
            // pub const PCRE2_DOLLAR_ENDONLY: u32 = 16;
            // pub const PCRE2_DOTALL: u32 = 32;
            // pub const PCRE2_DUPNAMES: u32 = 64;
            // pub const PCRE2_EXTENDED: u32 = 128;
            // pub const PCRE2_FIRSTLINE: u32 = 256;
            // pub const PCRE2_MATCH_UNSET_BACKREF: u32 = 512;
            // pub const PCRE2_MULTILINE: u32 = 1024;
            // pub const PCRE2_NEVER_UCP: u32 = 2048;
            // pub const PCRE2_NEVER_UTF: u32 = 4096;
            // pub const PCRE2_NO_AUTO_CAPTURE: u32 = 8192;
            // pub const PCRE2_NO_AUTO_POSSESS: u32 = 16384;
            // pub const PCRE2_NO_DOTSTAR_ANCHOR: u32 = 32768;
            // pub const PCRE2_NO_START_OPTIMIZE: u32 = 65536;
            // pub const PCRE2_UCP: u32 = 131072;
            // pub const PCRE2_UNGREEDY: u32 = 262144;
            // pub const PCRE2_UTF: u32 = 524288;
            let re = pcre2_compile_8(
                pattern.as_ptr(), /* the pattern */
                pattern.len(),    /* indicates pattern is zero-terminated */
                0,                /* default options */
                &mut err_number,  /* for error number */
                &mut err_offset,  /* for error offset */
                ptr::null_mut(),  /* use default compile context */
            );
            println!(
                "===> re:{:?},err_number:{:?},err_offset:{:?}",
                re, err_number, err_offset
            );
            if re == ptr::null_mut() {
                println!("===> re:{:?}", re);
                return;
            }
            // if re == ptr::null_mut::<pcre2_code_8>() {
            //     let buf: [u8; 256] = [0; 256];
            //     let buffer = buf.as_ptr() as *mut u8;
            //     pcre2_get_error_message_8(0, buffer, 256);
            //     println!(
            //         "PCRE2 compilation failed at offset {:?}: {:?}",
            //         err_offset, buf
            //     );
            //     return;
            // }
            //extern "C" {
            //     pub fn pcre2_match_8(
            //         arg1: *const pcre2_code_8,
            //         arg2: PCRE2_SPTR8,
            //         arg3: usize,
            //         arg4: usize,
            //         arg5: u32,
            //         arg6: *mut pcre2_match_data_8,
            //         arg7: *mut pcre2_match_context_8,
            //     ) -> ::std::os::raw::c_int;
            // }
            let subject = src.as_str();
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
            println!("===> rc:{:?}", rc);
            //
            if rc < 0 {
                match rc {
                    PCRE2_ERROR_NOMATCH => {
                        println!("No match");
                    }
                    _ => {
                        println!("Matching error {:?}", rc);
                    }
                }
                pcre2_match_data_free_8(match_data); /* Release memory used for the match */
                pcre2_code_free_8(re); /*   data and the compiled pattern. */
                return;
            }
            //*mut usize;
            let output_vector = pcre2_get_ovector_pointer_8(match_data);
            if rc == 0 {
                println!("output vector was not big enough for all the captured substrings");
            }
            let idx1 = *output_vector;
            let idx2 = *output_vector.offset(1);
            println!("idx1:{:?},idx2:{:?}", idx1, idx2);

            if idx1 > idx2 {
                println!("from end to start the match was: {:?}", &src[idx2..idx1]);
                return;
            }
            println!("====> {:?}", &src[idx1..idx2])
        }
    }
    #[test]
    fn match_and_sendUS() {
        unsafe {
            let pattern = r"\d{4}([^0-9^\s]{3,11})\S";
            let src= String::from("a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999");
            let mut err_number = 0;
            let mut err_offset = 0;
            let re = pcre2_compile_8(
                pattern.as_ptr(), /* the pattern */
                pattern.len(),    /* indicates pattern is zero-terminated */
                0,                /* default options */
                &mut err_number,  /* for error number */
                &mut err_offset,  /* for error offset */
                ptr::null_mut(),  /* use default compile context */
            );
            if re == ptr::null_mut() {
                println!(
                    "===> re:{:?},err_number:{:?},err_offset:{:?}",
                    re, err_number, err_offset
                );
                return;
            }
            let subject = src.as_str();
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
                match rc {
                    PCRE2_ERROR_NOMATCH => {
                        println!("No match");
                    }
                    _ => {
                        println!("Matching error {:?}", rc);
                    }
                }
                pcre2_match_data_free_8(match_data); /* Release memory used for the match */
                pcre2_code_free_8(re); /*   data and the compiled pattern. */
                return;
            }
            //*mut usize;
            let output_vector = pcre2_get_ovector_pointer_8(match_data);
            if rc == 0 {
                println!("output vector was not big enough for all the captured substrings");
            }
            let idx1 = *output_vector;
            let idx2 = *output_vector.offset(1);
            let idx3 = *output_vector.offset(2);
            let idx4 = *output_vector.offset(3);
            let idx5 = *output_vector.offset(4);
            let idx6 = *output_vector.offset(5);
            println!("====> {:?}", idx5);
            if idx1 > idx2 {
                println!("from end to start the match was: {:?}", &src[idx2..idx1]);
                pcre2_match_data_free_8(match_data); /* Release memory used for the match */
                pcre2_code_free_8(re); /*   data and the compiled pattern. */
                return;
            }
            println!("====> {:?}", &src[idx1..idx2]);
            println!("====> {:?}", &src[idx3..idx4]);
            println!("====> {:?}", &src[idx5..idx6]);
            // sendMsg(s)
        }
    }

    #[test]
    fn test_match() {
        let pattern = r"\d{4}([^0-9^\s]{3,11})\S";
        let src= "a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
        let ret = match_string(pattern, src);
        match ret {
            Ok(v) => {
                for v in v {
                    println!("{}", v);
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn sendMsg(s: &str) {
    let client = UdpSocket::bind("127.0.0.1:34254").unwrap();
    client.connect("127.0.0.1:3000").unwrap();
    client.send(s.as_ref()).unwrap();
}

type Result = result::Result<Vec<String>, String>;

fn match_string(pattern: &str, subject: &str) -> Result {
    unsafe {
        let mut err_number = 0;
        let mut err_offset = 0;
        let re = pcre2_compile_8(
            pattern.as_ptr(), /* the pattern */
            pattern.len(),    /* indicates pattern is zero-terminated */
            0,                /* default options */
            &mut err_number,  /* for error number */
            &mut err_offset,  /* for error offset */
            ptr::null_mut(),  /* use default compile context */
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
        //*mut usize;
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

            ret.push(subject[idx1..idx2].to_string());
            i += 2;
        }
        return Ok(ret);
    }
}
