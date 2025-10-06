//! Emulation Zbs

#![no_std]
// TODO: FIX AND REMOVE IT!!!
#![allow(static_mut_refs)]

use hikami_core::HYPERVISOR_DATA;
use hikami_core::emulate_extension::EmulateExtension;

use core::cell::OnceCell;
use raki::{Instruction, OpcodeKind, ZbsOpcode, ZicsrOpcode};
use spin::Mutex;

/// Singleton for Zbs.
pub static mut ZBS_DATA: Mutex<OnceCell<Zbs>> = Mutex::new(OnceCell::new());

/// Singleton for Zbs extension
pub struct Zbs;

impl Zbs {
    /// Constructor for `Zbs`.
    pub fn new() -> Self {
        Zbs
    }
}

impl EmulateExtension for Zbs {
    /// Emulate Zbs instruction.
    #[allow(clippy::cast_possible_truncation)]
    fn instruction(&mut self, inst: &Instruction) {
        let mut context = unsafe { HYPERVISOR_DATA.lock() }
            .get()
            .unwrap()
            .guest()
            .context;

        match inst.opc {
            OpcodeKind::Zbs(ZbsOpcode::BCLRI) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BEXTI) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BINVI) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BSETI) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BCLR) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BEXT) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BINV) => todo!(),
            OpcodeKind::Zbs(ZbsOpcode::BSET) => todo!(),
            _ => unreachable!(),
        }
    }

    /// Emulate Zicfiss CSRs access.
    fn csr(&mut self, inst: &Instruction) {
        let hypervisor_data = unsafe { HYPERVISOR_DATA.lock() };
        let mut context = hypervisor_data.get().unwrap().guest().context;

        let csr_num = inst.rs2.unwrap();
        match csr_num {
            unsupported_csr_num => {
                unimplemented!("unsupported CSRs: {unsupported_csr_num:#x}")
            }
        }
    }

    /// Emulate CSR field that already exists.
    fn csr_field(&mut self, _inst: &Instruction) {
        todo!("Implementing Zbs CSR field emulation");
    }

    /// Return whether given csr value is defined in the extension.
    ///
    /// This function returns `false` always because there is no CSR to emulate.
    fn is_csr_defined(&self, _: u16) -> bool {
        false
    }

    /// Return whether given csr value has newly defined field.
    ///
    /// This function returns `false` always because there is no CSR to emulate fields.
    fn is_csr_field_defined(&self, _: u16) -> bool {
        todo!("Implementing Zbs CSR field definition");
    }
}
