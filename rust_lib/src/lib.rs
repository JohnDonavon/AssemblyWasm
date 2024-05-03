use miden_stdlib::StdLibrary;
use miden_lib::MidenLib;
use miden_vm::Assembler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile() -> Result<(), JsValue> {
    let program = "use.std::math::u64

        begin
            push.1.0
            push.2.0
            exec.u64::overflowing_add
        end";

    let kernel = include_str!("api.masm");

    let assembler = Assembler::default()
        .with_library(&MidenLib::default())
        .expect("failed to load miden-lib")
        .with_library(&StdLibrary::default())
        .expect("failed to load std-lib")
        .with_kernel(kernel)
        .expect("kernel must be well formed");

    assembler
        .compile(program)
        .map_err(|err| format!("Failed to assemble program - {}", err))?;

    Ok(())
}
