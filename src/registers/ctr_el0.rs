// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2023 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>

//! System Control Register - EL1
//!
//! Provides top level control of the system, including its memory system, at EL1 and EL0.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
};

register_bitfields! {u64,
    pub CTR_EL0 [
        /// Traps EL0 execution of cache maintenance instructions to EL1, from AArch64 state only.
        ///
        /// 0 Any attempt to execute a DC CVAU, DC CIVAC, DC CVAC, DC CVAP, or IC IVAU
        ///   instruction at EL0 using AArch64 is trapped to EL1.
        /// 1 This control does not cause any instructions to be trapped.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on execution at EL0.
        ///
        /// If the Point of Coherency is before any level of data cache, it is IMPLEMENTATION DEFINED whether
        /// the execution of any data or unified cache clean, or clean and invalidate instruction that operates by
        /// VA to the point of coherency can be trapped when the value of this control is 1.
        ///
        /// If the Point of Unification is before any level of data cache, it is IMPLEMENTATION DEFINED whether
        /// the execution of any data or unified cache clean by VA to the point of unification instruction can be
        /// trapped when the value of this control is 1.
        ///
        /// If the Point of Unification is before any level of instruction cache, it is IMPLEMENTATION DEFINED
        /// whether the execution of any instruction cache invalidate by VA to the point of unification
        /// instruction can be trapped when the value of this control is 1.
        TminLine OFFSET(32) NUMBITS(6) [],

        /// Endianness of data accesses at EL1, and stage 1 translation table walks in the EL1&0 translation regime.
        ///
        /// 0 Explicit data accesses at EL1, and stage 1 translation table walks in the EL1&0
        ///   translation regime are little-endian.
        /// 1 Explicit data accesses at EL1, and stage 1 translation table walks in the EL1&0
        ///   translation regime are big-endian.
        ///
        /// If an implementation does not provide Big-endian support at Exception Levels higher than EL0, this
        /// bit is RES 0.
        ///
        /// If an implementation does not provide Little-endian support at Exception Levels higher than EL0,
        /// this bit is RES 1.
        ///
        /// The EE bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on the PE.
        DIC OFFSET(29) NUMBITS(1) [],

        /// Endianness of data accesses at EL0.
        ///
        /// 0 Explicit data accesses at EL0 are little-endian.
        ///
        /// 1 Explicit data accesses at EL0 are big-endian.
        ///
        /// If an implementation only supports Little-endian accesses at EL0 then this bit is RES 0. This option
        /// is not permitted when SCTLR_EL1.EE is RES 1.
        ///
        /// If an implementation only supports Big-endian accesses at EL0 then this bit is RES 1. This option is
        /// not permitted when SCTLR_EL1.EE is RES 0.
        ///
        /// This bit has no effect on the endianness of LDTR , LDTRH , LDTRSH , LDTRSW , STTR , and STTRH instructions
        /// executed at EL1.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on execution at EL0.
        IDC OFFSET(28) NUMBITS(1) [],

        /// Write permission implies XN (Execute-never). For the EL1&0 translation regime, this bit can force
        /// all memory regions that are writable to be treated as XN.
        ///
        /// 0 This control has no effect on memory access permissions.
        ///
        /// 1 Any region that is writable in the EL1&0 translation regime is forced to XN for accesses
        ///   from software executing at EL1 or EL0.
        ///
        /// The WXN bit is permitted to be cached in a TLB.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on the PE.
        CWG OFFSET(24) NUMBITS(4) [],

        /// Traps EL0 execution of WFE instructions to EL1, from both Execution states.
        ///
        /// 0 Any attempt to execute a WFE instruction at EL0 is trapped to EL1, if the instruction
        ///   would otherwise have caused the PE to enter a low-power state.
        ///
        /// 1 This control does not cause any instructions to be trapped.
        ///
        /// In AArch32 state, the attempted execution of a conditional WFE instruction is only trapped if the
        /// instruction passes its condition code check.
        ///
        /// **Note:**
        ///
        /// Since a WFE or WFI can complete at any time, even without a Wakeup event, the traps on WFE of
        /// WFI are not guaranteed to be taken, even if the WFE or WFI is executed when there is no Wakeup
        /// event. The only guarantee is that if the instruction does not complete in finite time in the
        /// absence of a Wakeup event, the trap will be taken.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on execution at EL0.
        ERG OFFSET(20) NUMBITS(4) [],

        /// Traps EL0 executions of WFI instructions to EL1, from both execution states:
        ///
        /// 0 Any attempt to execute a WFI instruction at EL0 is trapped EL1, if the instruction would
        ///   otherwise have caused the PE to enter a low-power state.
        ///
        /// 1 This control does not cause any instructions to be trapped.
        ///
        /// In AArch32 state, the attempted execution of a conditional WFI instruction is only trapped if the
        /// instruction passes its condition code check.
        ///
        /// **Note:**
        ///
        /// Since a WFE or WFI can complete at any time, even without a Wakeup event, the traps on WFE of
        /// WFI are not guaranteed to be taken, even if the WFE or WFI is executed when there is no Wakeup
        /// event. The only guarantee is that if the instruction does not complete in finite time in the
        /// absence of a Wakeup event, the trap will be taken.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on execution at EL0.
        DminLine OFFSET(16) NUMBITS(4) [],

        /// Traps EL0 accesses to the CTR_EL0 to EL1, from AArch64 state only.
        ///
        /// 0 Accesses to the CTR_EL0 from EL0 using AArch64 are trapped to EL1.
        ///
        /// 1 This control does not cause any instructions to be trapped.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on execution at EL0.
        L1Ip OFFSET(14) NUMBITS(2) [
            Reserved = 0b00,
            AIVIVT = 0b01,
            VIPT = 0b10,
            PIPT = 0b11,
        ],

        /// Traps EL0 execution of DC ZVA instructions to EL1, from AArch64 state only.
        ///
        /// 0 Any attempt to execute a DC ZVA instruction at EL0 using AArch64 is trapped to EL1.
        ///   Reading DCZID_EL0.DZP from EL0 returns 1, indicating that DC ZVA instructions
        ///   are not supported.
        ///
        /// 1 This control does not cause any instructions to be trapped.
        ///
        /// When ARMv8.1-VHE is implemented, and the value of HCR_EL2.{E2H, TGE} is {1, 1}, this bit
        /// has no effect on execution at EL0.
        IminLine OFFSET(0) NUMBITS(4) [],

    ]
}

pub struct Reg;

impl Readable for Reg {
    type T = u64;
    type R = CTR_EL0::Register;

    sys_coproc_read_raw!(u64, "CTR_EL0", "x");
}

impl Writeable for Reg {
    type T = u64;
    type R = CTR_EL0::Register;

    sys_coproc_write_raw!(u64, "CTR_EL0", "x");
}

pub const CTR_EL0: Reg = Reg {};
