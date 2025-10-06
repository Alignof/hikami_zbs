//! Emulation Zbs

#![no_std]
// TODO: FIX AND REMOVE IT!!!
#![allow(static_mut_refs)]

use hikami_core::HYPERVISOR_DATA;
use hikami_core::emulate_extension::EmulateExtension;

use core::cell::OnceCell;
use raki::{Instruction, OpcodeKind, ZbsOpcode};
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
            OpcodeKind::Zbs(ZbsOpcode::BCLRI) => {
                let input = context.xreg(inst.rs1.unwrap()) as usize;
                let shamt = inst.imm.unwrap();
                let output = input & !(1 << shamt);
                context.set_xreg(inst.rd.unwrap(), output as u64);
            }
            OpcodeKind::Zbs(ZbsOpcode::BEXTI) => {
                let input = context.xreg(inst.rs1.unwrap()) as usize;
                let shamt = inst.imm.unwrap();
                let output = (input >> shamt) & 1;
                context.set_xreg(inst.rd.unwrap(), output as u64);
            }
            OpcodeKind::Zbs(ZbsOpcode::BINVI) => {
                let input = context.xreg(inst.rs1.unwrap()) as usize;
                let shamt = inst.imm.unwrap();
                let output = input ^ (1 << shamt);
                context.set_xreg(inst.rd.unwrap(), output as u64);
            }
            OpcodeKind::Zbs(ZbsOpcode::BSETI) => {
                let input = context.xreg(inst.rs1.unwrap()) as usize;
                let shamt = inst.imm.unwrap();
                let output = input | (1 << shamt);
                context.set_xreg(inst.rd.unwrap(), output as u64);
            }
            OpcodeKind::Zbs(ZbsOpcode::BCLR) => {
                let rs1 = context.xreg(inst.rs1.unwrap());
                let rs2 = context.xreg(inst.rs2.unwrap());
                context.set_xreg(inst.rd.unwrap(), rs1 & !(1 << rs2));
            }
            OpcodeKind::Zbs(ZbsOpcode::BEXT) => {
                let rs1 = context.xreg(inst.rs1.unwrap());
                let rs2 = context.xreg(inst.rs2.unwrap());
                context.set_xreg(inst.rd.unwrap(), (rs1 >> rs2) & 1);
            }
            OpcodeKind::Zbs(ZbsOpcode::BINV) => {
                let rs1 = context.xreg(inst.rs1.unwrap());
                let rs2 = context.xreg(inst.rs2.unwrap());
                context.set_xreg(inst.rd.unwrap(), rs1 ^ (1 << rs2));
            }
            OpcodeKind::Zbs(ZbsOpcode::BSET) => {
                let rs1 = context.xreg(inst.rs1.unwrap());
                let rs2 = context.xreg(inst.rs2.unwrap());
                context.set_xreg(inst.rd.unwrap(), rs1 | (1 << rs2));
            }
            _ => unreachable!(),
        }
    }

    /// Emulate Zicfiss CSRs access.
    fn csr(&mut self, _inst: &Instruction) {
        unreachable!();
    }

    /// Emulate CSR field that already exists.
    fn csr_field(&mut self, _inst: &Instruction) {
        unreachable!();
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
        false
    }
}
