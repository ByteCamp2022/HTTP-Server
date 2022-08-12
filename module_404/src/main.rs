wit_bindgen_rust::import!("../imports.wit");
wit_bindgen_rust::export!("../exports.wit");


struct Exports;

impl exports::Exports for Exports {

    fn proxy(name: String, param: String) -> String{
        println!("\tUsing module_404");
        match name.as_str() {
            "responseStatus" => {
                return responseStatus();
            },
            "response_HTML" => {
                return response_HTML(&param);
            }
            "response" => {
                return response(&param);
            },
            _ => {
                "no such a func !".to_string()
            }
        }
    }

}

fn responseStatus() -> String {
    let status = String::from("HTTP/1.1 404 NOT FOUND");
    return imports::proxy("responseStatus", &status[..]);
}

fn response_HTML(path: &str) -> String {
    return imports::proxy("response_HTML", path);
}

fn response(path: &str) -> String {
    println!("\t\tUsing function: response(path: String) in module_404");
    let status = responseStatus();
    let contents = response_HTML(path);
    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    return imports::proxy("response", &resp[..]);
}

fn main() {
    println!("Hello, world!");
}
