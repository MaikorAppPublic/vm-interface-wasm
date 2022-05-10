use lazy_static::lazy_static;
use maikor_asm_parser::line::parse_line;
use maikor_language::mem::address::{
    ATLAS1_BANK_ID, ATLAS2_BANK_ID, CODE_BANK_ID, RAM_BANK_ID, SAVE_BANK_ID,
};
use maikor_vm_interface::VMHost;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref VM_INSTANCE: Mutex<VMHost> = Mutex::new(VMHost::new());
}

#[wasm_bindgen]
pub fn set_byte_register(addr: usize, value: u8) {
    VM_INSTANCE.lock().unwrap().vm.registers[addr] = value;
}

#[wasm_bindgen]
pub fn set_word_register(addr: usize, value: u16) {
    let bytes = value.to_be_bytes();
    VM_INSTANCE.lock().unwrap().vm.registers[addr] = bytes[0];
    VM_INSTANCE.lock().unwrap().vm.registers[addr + 1] = bytes[1];
}

#[wasm_bindgen]
pub fn set_byte_mem(addr: usize, value: u8) {
    VM_INSTANCE.lock().unwrap().vm.memory[addr] = value;
}

#[wasm_bindgen]
pub fn set_word_mem(addr: usize, value: u16) {
    let bytes = value.to_be_bytes();
    VM_INSTANCE.lock().unwrap().vm.memory[addr] = bytes[0];
    VM_INSTANCE.lock().unwrap().vm.memory[addr + 1] = bytes[1];
}

// Order is registers, pc, fp, sp, code_bank_id, ram_bank_id, atlas1_bank_id, atlas1_bank_id, save_bank_id
#[wasm_bindgen]
pub fn get_core() -> Vec<u8> {
    let host = VM_INSTANCE.lock().unwrap();
    let mut list = host.vm.registers.to_vec();
    list.extend_from_slice(&host.vm.pc.to_be_bytes());
    list.extend_from_slice(&host.vm.get_sp().to_be_bytes());
    list.extend_from_slice(&host.vm.get_fp().to_be_bytes());
    list.push(host.vm.memory[CODE_BANK_ID as usize]);
    list.push(host.vm.memory[CODE_BANK_ID as usize + 1]);
    list.push(host.vm.memory[RAM_BANK_ID as usize]);
    list.push(host.vm.memory[RAM_BANK_ID as usize + 1]);
    list.push(host.vm.memory[ATLAS1_BANK_ID as usize]);
    list.push(host.vm.memory[ATLAS1_BANK_ID as usize + 1]);
    list.push(host.vm.memory[ATLAS2_BANK_ID as usize]);
    list.push(host.vm.memory[ATLAS2_BANK_ID as usize + 1]);
    list.push(host.vm.memory[SAVE_BANK_ID as usize]);
    list
}

#[wasm_bindgen]
pub fn get_memory(start: u16, len: u8) -> Vec<u8> {
    VM_INSTANCE.lock().unwrap().vm.memory[start as usize..start as usize + len as usize].to_vec()
}

#[wasm_bindgen]
pub fn execute_statement(text: String) -> Option<String> {
    match parse_line(&text) {
        Ok((op, arg_bytes)) => {
            let mut bytes = vec![op];
            bytes.extend_from_slice(&arg_bytes);
            VM_INSTANCE.lock().unwrap().vm.execute_op(&bytes);
            None
        }
        Err(err) => Some(err.to_string()),
    }
}
