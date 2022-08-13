mod a1_file;
mod demo1;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Index;


    use crate::{Payload, Root};
    use crate::a1_file::{read_file1, read_file2};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_file() {
        assert_eq!(
            read_file1().unwrap_err().to_string(),
            read_file2().unwrap_err().to_string()
        );
        println!("Success!")
    }

    #[test]
    fn test_json() {
        let mut parsed = json::parse(r#"
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}
"#).unwrap();
        let o = Root {
            code: 0,
            success: false,
            payload: Payload {
                features: Vec::new()
            },
        };
        let s: HashMap<String, String> = parsed.entries().collect();
        println!("{:?}", s)
        // match parsed {
        //     json::JsonValue::String(c) => {
        //         println!("String")
        //     }
        //     json::JsonValue::Array(..) => {
        //         println!("Array")
        //     }
        //     json::JsonValue::Boolean(..) => {
        //         println!("Boolean")
        //     }
        //     json::JsonValue::Number(..) => {
        //         println!("Number")
        //     }
        //     json::JsonValue::Object(o) => {
        //        parsed.take();
        //     }
        //     json::JsonValue::Short(..) => {
        //         println!("Short")
        //     }
        //     _ => {println!("asdasd") }
        // }
    }
}


pub struct Payload {
    pub features: Vec<String>,
}

pub struct Root {
    pub code: i64,
    pub success: bool,
    pub payload: Payload,
}