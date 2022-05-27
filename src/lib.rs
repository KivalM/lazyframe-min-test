use std::{io::Cursor, panic};

use polars::{prelude::{CsvReader, col, IntoLazy}, io::SerReader};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console::log_1;

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn init_hooks() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}


#[wasm_bindgen]
pub fn start(buff: &[u8],
    item_id:&str, 
    order_id:&str,
    item_name:&str) -> JsValue{
    let cursor = Cursor::new(buff);

    let lf = CsvReader::new(cursor).with_ignore_parser_errors(true).finish().unwrap().lazy();

    log_1(&JsValue::from("Read CSV"));


    let df = lf.groupby([col(order_id)]);
    log_1(&JsValue::from("grouped lf"));
    let df = df.agg([col(item_id),col(item_name)]);

    log_1(&JsValue::from("agg lf"));

    // Error occurs here
    let df = df.collect().unwrap();
    log_1(&JsValue::from("collected lf"));

    JsValue::from(format!("{:?}",df.head(Some(10))))

}

// I am working with Polars in a wasm environment provided by `https://github.com/universalmind303/polars/tree/wasm`

// I have noticed an inconsistency with the LazyFrame.collect operation where it sometimes creates threads when working with certain datasets. 

// Here is the code that relates to the issue
// ```rust
// #[wasm_bindgen]
// pub fn start(buff: &[u8],
//     item_id:&str, 
//     order_id:&str,
//     item_name:&str) -> JsValue{

//     let cursor = Cursor::new(buff);
//     let lf = CsvReader::new(cursor).with_ignore_parser_errors(true).finish().unwrap().lazy();


//     let df = lf.groupby([col(order_id)]);
//     let df = df.agg([col(item_id),col(item_name)]);
    
//     // Error occurs here
//     let df = df.collect().unwrap();

// }```

