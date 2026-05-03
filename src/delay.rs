use core::arch::asm;

#[allow(dead_code)]
pub fn delay(ms: u32) {
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe { asm!("nop") }
        }
    }
}
