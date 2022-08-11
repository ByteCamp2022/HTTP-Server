use std::{collections::HashMap, hash::Hash};
use lazy_static::*;
use std::sync::Mutex;
use anyhow::Result;
use wasmtime::{*};
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

wit_bindgen_wasmtime::export!("../imports.wit");
wit_bindgen_wasmtime::import!("../exports.wit");


use imports::*;
use exports::*;

fn responseStatus(s: String) -> String {
    let status = String::from("HTTP/1.1 200 OK");
    return status;
}

fn response_HTML(path: String) -> String {
    let html = fs::read_to_string(path).unwrap();
    return html;
}

fn response(path: String) -> String {
    let s = String::from(" ");
    let status = responseStatus(s);
    let contents = response_HTML(path);
    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    return resp;
}

#[derive(Default)]
pub struct MyImports;


// type FuncType = fn(&String)->&String;
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, fn(String)->String>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };

    static ref MODULE_FUNC: Mutex<HashMap<String, (Exports<Context<MyImports, ExportsData>>, Store<Context<MyImports, ExportsData>>)>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}


struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
    exports: E,
}


fn default_config() -> Result<Config> {
    // Create an engine with caching enabled to assist with iteration in this
    // project.
    let mut config = Config::new();
    config.cache_config_load_default()?;
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    Ok(config)
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}


fn instantiate<I: Default, E: Default, T>(
    wasm: &str,
    add_imports: impl FnOnce(&mut Linker<Context<I, E>>) -> Result<()>,
    mk_exports: impl FnOnce(
        &mut Store<Context<I, E>>,
        &Module,
        &mut Linker<Context<I, E>>,
    ) -> Result<(T, Instance)>,
) -> Result<(T, Store<Context<I, E>>)> {
    let engine = Engine::new(&default_config()?)?;
    let module = Module::from_file(&engine, wasm)?;

    let mut linker = Linker::new(&engine);
    add_imports(&mut linker)?; //装载我们的实现到linker
    wasmtime_wasi::add_to_linker(&mut linker, |cx| &mut cx.wasi)?;


    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            imports: I::default(),
            exports: E::default(),
        },
    );
    let (exports, _instance) = mk_exports(&mut store, &module, &mut linker)?;

    // for (key, value, _) in linker.iter(&mut store) {
    //     println!("{} / {}", key, value);

    // }

    Ok((exports, store))
}


fn registry(name: &str, f: fn(String)->String) {
    {
        let mut map = HASHMAP.lock().unwrap();
        map.insert(String::from(name), f);
    }
}

fn registry_module(path: &str, name: &str) -> Result<()> {
    let (e, mut s) = instantiate(
        path,
        |linker| imports::add_to_linker(linker, |cx| -> &mut MyImports { &mut cx.imports }),
        |store, module, linker| Exports::instantiate(store, module, linker, |cx| &mut cx.exports),
    )?;
    {
        let mut map = MODULE_FUNC.lock().unwrap();
        map.insert(String::from(name), (e, s));
    }
    Ok(())
}

fn call_module_func(mname: &str, fname: &str, param: &str) -> String {

    let mut map = MODULE_FUNC.lock().unwrap();
    let (e, mut s) = map.remove(mname).unwrap();
    let rs = e.proxy(&mut s, fname, param);      
    map.insert(String::from(mname), (e, s));

    rs.unwrap()
}

impl Imports for MyImports {
    // 暴露给wasm module的函数

    fn proxy(&mut self, name: &str, param: &str) -> String {
        let mut map = HASHMAP.lock().unwrap();
        let param = String::from(param);

        let rs = map.get(name).unwrap()(param);
        return rs;
    }

}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        
        streram_handle(&mut stream);
    }




    Ok(())
}

fn streram_handle(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
   
    registry("responseStatus", responseStatus);
    registry("response_HTML", response_HTML);
    registry("response", response);

    registry_module("module_200.wasm", "module_200");
    let status =  call_module_func("module_200", "responseStatus", " ");
    let contents = call_module_func("module_200", "response_HTML", "hello.html");
    let resp = call_module_func("module_200", "response", "hello.html");
    println!("{}", resp);

    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
