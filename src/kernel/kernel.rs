// src/kernel/kernel.rs
use core::arch::asm;
use crate::kernel::tty::tty;
use crate::hal::cpu;
use crate::{print, println};

#[no_mangle]
pub extern "C" fn _kernel_init() {
    // TODO: 加載 GDT OK
    // TODO: 加載 IDT OK
    // TODO: 啟用分頁
    tty::tty_init(tty::VGA_BUFFER_PADDR);
    tty::tty_set_theme(tty::VGA_COLOR_WHITE, tty::VGA_COLOR_BLACK);
}

#[no_mangle]
pub extern "C" fn _kernel_post_init() {

}

#[no_mangle]
pub extern "C" fn _kernel_main() {

    println!("Loading....!");
    
    // tty::tty_clear();
    
    println!("Welcome to Cure OS!");

    println!("{0} + {1} = {0}", 1, 2);

    let name = "111";
    let age = 25;
    println!("{}, {}", name, age);

    let value = 42;
    println!("10: {}, 16: {:x}", value, value);

    let mut brand_buffer = [0u8; 64];
    println!("{}", cpu::cpu_get_brand(&mut brand_buffer));
    println!("{}", cpu::cpu_get_model(&mut brand_buffer));
    
    unsafe {
        let eflags: u32;
        asm!("pushfd; pop {}", out(reg) eflags);
        println!("Current EFLAGS: 0x{:x}", eflags);
    }

    // unsafe {
    //     core::arch::asm!(
    //         "movl $0, %eax",
    //         "movl $0, %ebx",
    //         "divl %ebx",
    //         options(att_syntax)
    //     );
    // }

    unsafe {
        asm!(
            "mov ax, 0x27",
            "mov ds, ax",
            options(nostack)
        );
    }

    // cpu::cpu_enable_interrupts();

    // unsafe {
    //     core::arch::asm!(
    //         "int $13",
    //         options(att_syntax)
    //     );
    // }

    loop {
        cpu::cpu_idle();
    }
}