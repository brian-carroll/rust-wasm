// Simple example of how to use parity-wasm builder api.
// Builder api introduced as a method for fast generation of
// different small wasm modules.
use std::fs;
use std::ffi::CString;
use std::ptr;
use std::slice;

use binaryen::ffi::*;
// use parity_wasm::{builder, elements};
use wasmer::{Store, Module, Instance, Value, imports};

fn create_wasm_bytes() -> Vec<u8> {
    unsafe {
        let module = BinaryenModuleCreate();
        let name = CString::new("add_one").unwrap();
        let params = BinaryenTypeInt32();
        let results = BinaryenTypeInt32();
        let var_types = ptr::null_mut();
        let num_var_types = 0;
        let body = BinaryenBinary(module,
            BinaryenAddInt32(),
            BinaryenLocalGet(module, 0, BinaryenTypeInt32()),
            BinaryenConst(module, BinaryenLiteralInt32(2))
        );

        BinaryenAddFunction(
            module,
            name.as_ptr(),
            params,
            results,
            var_types,
            num_var_types,
            body
        );

        BinaryenAddFunctionExport(
            module, name.as_ptr(), name.as_ptr()
        );

        let result = BinaryenModuleAllocateAndWrite(module, ptr::null());
        let bytes = slice::from_raw_parts(result.binary as *const u8, result.binaryBytes);
        return bytes.to_vec();
    }
}


fn main() -> Result<(), String> {
    let wasm_bytes: Vec<u8> = create_wasm_bytes();

    match fs::write("test.wasm", &wasm_bytes) {
        Ok(()) => (),
        Err(e) => return Err(format!("writing Wasm file {:?}", e)),
    };

    // Based on the example at https://docs.rs/wasmer/2.0.0/wasmer/
    let store = Store::default();
    let module = match Module::from_binary(&store, &wasm_bytes) {
        Ok(m) => m,
        Err(e) => return Err(format!("loading Wasm module {:?}", e)),
    };

    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};

    let instance = match Instance::new(&module, &import_object) {
        Ok(inst) => inst,
        Err(e) => return Err(format!("instanciating Wasm module {:?}", e)),
    };

    let add_one = match instance.exports.get_function("add_one") {
        Ok(f) => f,
        Err(e) => return Err(format!("getting exported Wasm function {:?}", e)),
    };

    let result = match add_one.call(&[Value::I32(42)]) {
        Ok(f) => f,
        Err(e) => return Err(format!("calling export Wasm function {:?}", e)),
    };

    assert_eq!(result[0], Value::I32(43));

    return Ok(());
}