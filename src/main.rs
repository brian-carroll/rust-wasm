// Simple example of how to use parity-wasm builder api.
// Builder api introduced as a method for fast generation of
// different small wasm modules.
use std::fs;

extern crate parity_wasm;
use parity_wasm::{builder, elements};
use wasmer::{Store, Module, Instance, Value, imports};

fn create_wasm_bytes() -> Vec<u8> {
    // Based on the example at https://github.com/paritytech/parity-wasm/blob/master/examples/build.rs
    let module = builder::module()
        .function()
            .signature()
                .with_param(elements::ValueType::I32)
                .with_result(elements::ValueType::I32)
                .build()
            .body()
                .with_instructions(elements::Instructions::new(vec![
                    elements::Instruction::GetLocal(0),
                    elements::Instruction::I32Const(1),
                    elements::Instruction::I32Add,
                    elements::Instruction::End,
                ]))
                .build()
            .build()
        .export()
            .field("add_one")
            .internal().func(0)
            .build()
        .build();

    return parity_wasm::elements::serialize(module).unwrap();
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