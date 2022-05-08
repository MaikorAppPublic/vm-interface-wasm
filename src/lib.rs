use std::sync::Mutex;
use lazy_static::lazy_static;
use maikor_asm_parser::line::parse_line;
use maikor_asm_parser::ParserError;
use wasm_bindgen::prelude::*;
use maikor_vm_interface::VMHost;

lazy_static! {
    static ref VM_INSTANCE: Mutex<VMHost> = Mutex::new(VMHost::new());
}

#[wasm_bindgen]
pub fn set_register(addr: usize, value: u8) {
    VM_INSTANCE.lock().unwrap().vm.registers[addr] = value;
}

#[wasm_bindgen]
pub fn set_ext_register(addr: usize, value: u16) {
    let bytes = value.to_be_bytes();
    VM_INSTANCE.lock().unwrap().vm.registers[addr] = bytes[0];
    VM_INSTANCE.lock().unwrap().vm.registers[addr + 1] = bytes[1];
}

#[wasm_bindgen]
pub fn get_registers() -> Vec<u8> {
    VM_INSTANCE.lock().unwrap().vm.registers.to_vec()
}

#[wasm_bindgen]
pub fn execute_statement(text: String) -> Option<String> {
    match parse_line(&text) {
        Ok((op, mut bytes)) => {
            bytes.insert(0, op);
            VM_INSTANCE.lock().unwrap().vm.execute_op(&bytes);
            None
        }
        Err(err) => Some(err.to_string())
    }
}