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


fn main() -> anyhow::Result<()> {
    let wasm_bytes: Vec<u8> = create_wasm_bytes();

    fs::write("test.wasm", &wasm_bytes)?;

    // Based on the example at https://docs.rs/wasmer/2.0.0/wasmer/
    let store = Store::default();
    let module = Module::from_binary(&store, &wasm_bytes)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    return Ok(());
}