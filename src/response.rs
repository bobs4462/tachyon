pub struct Response {
    code: Vec<u8>,
    content: Vec<u8>,
    mime: Vec<u8>,
}

impl Response {
    pub fn new(code: Vec<u8>, content: Vec<u8>, mime: Vec<u8>) -> Self {
        Response {
            code,
            content,
            mime,
        }
    }

    pub fn compose(self) -> Vec<u8> {
        [
            "HTTP/1.1 ".as_bytes(),
            &self.code,
            "\r\ncontent-type: ".as_bytes(),
            &self.mime,
            "\r\ncontent-length: ".as_bytes(),
            format!("{}", self.content.len()).as_bytes(),
            "\r\n\r\n".as_bytes(),
            &self.content,
        ]
        .concat()
        .to_vec()
    }
}
