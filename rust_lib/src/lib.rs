use miden_stdlib::StdLibrary;
use miden_vm::Assembler;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile() -> Result<(), JsValue> {
    let program =
        "use.std::math::u64

        begin
            push.1.0
            push.2.0
            exec.u64::overflowing_add
        end";
    
    let assembler = Assembler::default()
        .with_library(&StdLibrary::default())
        .map_err(|err| format!("Failed to load stdlib - {}", err))?
        .with_debug_mode(true);

    assembler
        .compile(program)
        .map_err(|err| format!("Failed to assemble program - {}", err))?;
    
    Ok(())
}
