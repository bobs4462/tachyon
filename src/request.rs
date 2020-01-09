use serde_json::{self, Value};
use std::collections::HashMap;

pub struct Request<'a> {
    method: &'a [u8],
    path: &'a [u8],
    headers: HashMap<&'a [u8], &'a [u8]>,
    body: Value,
}

impl<'a> Request<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        println!("{:?}", buffer);
        let method;
        let path;
        let mut headers: HashMap<&'a [u8], &'a [u8]> = HashMap::with_capacity(17);
        let mut body: Value = json!(null);

        let mut i = 0;
        let mut marker = 0;
        // getting HTTP method
        while 0x20 != buffer[i] {
            i += 1;
        }
        method = &buffer[marker..i];

        // getting HTTP path
        i += 1;
        marker = i;
        while 0x20 != buffer[i] {
            i += 1;
        }
        path = &buffer[marker..i];

        // getting HTTP headers
        while 0xd != buffer[i] {
            i += 1;
        }
        i += 2;
        marker = i;
        loop {
            println!("{}", i);
            let key;
            let val;
            while 0x3a == buffer[i] {
                i += 1;
            }
            key = &buffer[marker..i];
            i += 1;
            marker = i;
            while 0xd == buffer[i] {
                i += 1;
            }
            val = &buffer[marker..i];
            headers.insert(key, val);
            i += 2;
            if buffer[i] == 0xd {
                i += 2;
                break;
            }
        }
        // if buffer[i] != 0x0 {
        //     println!("LAST {}", buffer[i]);
        //     marker = i;
        //     while 0x0 != buffer[i] {
        //         i += 1;
        //     }
        //     body = serde_json::from_slice(&buffer[marker..i]).unwrap();
        // }
        Request {
            method,
            path,
            headers,
            body,
        }
    }
}
