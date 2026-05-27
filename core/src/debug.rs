use crate::{cpu::Cpu, mmu::Mmu};

/// Debugger interface.
///
/// The users of this library can implement this interface to inspect the state of the emulator.
pub trait Debugger<'rom> {
    /// The function is called on the initialization phase.
    fn init(&mut self, cpu: &Cpu<Mmu<'rom>>);

    /// The function is called right before the emulator starts executing an instruction. Deprecated.
    fn take_cpu_snapshot(&mut self, cpu: Cpu<Mmu<'rom>>);

    /// Decode an instruction.
    fn on_decode(&mut self, cpu: &Cpu<Mmu<'rom>>);

    /// Check if the external signal is triggered. Deprecated.
    fn check_signal(&mut self);
}

impl dyn Debugger<'_> {
    /// Create an empty debugger.
    pub fn empty() -> NullDebugger {
        NullDebugger
    }
}

/// Empty debugger which does nothing.
pub struct NullDebugger;

impl Debugger<'_> for NullDebugger {
    fn init(&mut self, _: &Cpu<Mmu<'_>>) {}

    fn take_cpu_snapshot(&mut self, _: Cpu<Mmu<'_>>) {}

    fn on_decode(&mut self, _: &Cpu<Mmu<'_>>) {}

    fn check_signal(&mut self) {}
}
