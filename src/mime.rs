use std::collections::HashMap;

lazy_static! {
    pub static ref MIMES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("css", "text/css");
        map.insert("csv", "text/csv");
        map.insert("doc", "application/msword");
        map.insert("gif", "image/gif");
        map.insert("html", "text/html");
        map.insert("ico", "image/x-icon");
        map.insert("jpg", "image/jpeg");
        map.insert("png", "image/png");
        map.insert("js", "application/javascript");
        map.insert("json", "application/json");
        map.insert("odp", "application/vnd.oasis.opendocument.presentation");
        map.insert("ods", "application/vnd.oasis.opendocument.spreadsheet");
        map.insert("odt", "application/vnd.oasis.opendocument.text");
        map.insert("pdf", "application/pdf");
        map.insert("ppt", "application/vnd.ms-powerpoint");
        map.insert("sh", "application/x-sh");
        map.insert("svg", "image/svg+xml");
        map.insert("tar", "application/x-tar");
        map.insert("tiff", "image/tiff");
        map.insert("ttf", "font/ttf");
        map.insert("woff", "font/woff");
        map.insert("xhtml", "application/xhtml+xml");
        map.insert("xls", "application/vnd.ms-excel");
        map.insert("xml", "application/xml");
        map.insert("zip", "application/zip");
        map.insert("map", "application/octet-stream");
        map
    };
}
