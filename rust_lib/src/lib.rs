use assembly::AssemblyContext;
use miden_objects::accounts::AccountCode;
use miden_stdlib::StdLibrary;
use miden_lib::MidenLib;
use miden_vm::{Assembler, ModuleAst};
use wasm_bindgen::prelude::*;
use miden_lib::transaction::TransactionKernel;
use console_error_panic_hook::set_once as set_panic_hook;

#[wasm_bindgen]
pub fn compile() -> Result<(), JsValue> {
  // set_panic_hook();
  web_sys::console::log_1(&"Compiling".into());
  // PublicKey([1049635298189747957, 6231234849174198326, 10899808482641779242, 11586056744462738121])
  // let pub_key = rpo_falcon512::PublicKey::new([1049635298189747957, 6231234849174198326, 10899808482641779242, 11586056744462738121]);
    // let program = "use.std::math::u64

    //     begin
    //         push.1.0
    //         push.2.0
    //         exec.u64::overflowing_add
    //     end";
    let program = "
    use.miden::contracts::wallets::basic->basic_wallet
    use.miden::contracts::auth::basic

    export.basic_wallet::receive_asset
    export.basic_wallet::send_asset
    export.basic::auth_tx_rpo_falcon512";

    // let kernel = include_str!("api.masm");

    let assembler = TransactionKernel::assembler();
        // .with_library(&MidenLib::default())
        // .expect("failed to load miden-lib")
        // .with_library(&StdLibrary::default())
        // .expect("failed to load std-lib")
        // .with_kernel(kernel)
        // .expect("kernel must be well formed");
    let account_code_ast = ModuleAst::parse(program)
      .map_err(|err| format!("Failed to assemble program - {}", err))?;


    let account_code = AccountCode::new(account_code_ast.clone(), &assembler)
      .map_err(|err| format!("Failed to assemble program - {}", err))?;
    web_sys::console::log_1(&"Compiled".into());
    web_sys::console::log_1(&format!("{:?}", account_code).into());
    // assembler
    //   .compile_module(&account_code_ast, None, &mut AssemblyContext::for_module(false))
    //   .map_err(|err| format!("Failed to assemble program - {}", err))?;

    Ok(())
}
