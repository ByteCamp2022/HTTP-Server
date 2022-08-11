wit_bindgen_rust::import!("../imports.wit");
wit_bindgen_rust::export!("../exports.wit");



struct Exports;

impl exports::Exports for Exports {

    fn proxy(name: String, param: String) -> String{
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
    return imports::proxy("responseStatus", " ");
}

fn response_HTML(path: &str) -> String {
    return imports::proxy("response_HTML", path);
}

fn response(path: &str) -> String {
    return imports::proxy("response", path);
}

fn main() {
    println!("Hello, world!");
}
