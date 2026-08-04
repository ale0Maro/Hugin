#![no_std]
#![no_main]

mod panic {
    mod panic;
}

const PORT_COM1: u16 = 0x3F8;

// Allocazione di 16 KB di stack dedicato al Kernel (allineato a 16 byte per ABI SysV)
#[repr(C, align(16))]
struct KernelStack([u8; 16384]);
static KERNEL_STACK: KernelStack = KernelStack([0; 16384]);

#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            out("al") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    val
}

pub fn init_serial() {
    unsafe {
        outb(PORT_COM1 + 1, 0x00); // Disabilita interrupt UART
        outb(PORT_COM1 + 3, 0x80); // Abilita DLAB (baud rate)
        outb(PORT_COM1 + 0, 0x03); // 38400 baud (divisor 3)
        outb(PORT_COM1 + 1, 0x00);
        outb(PORT_COM1 + 3, 0x03); // 8 bit, no parity, 1 stop bit
        outb(PORT_COM1 + 2, 0xC7); // Abilita FIFO
        outb(PORT_COM1 + 4, 0x0B); // Abilita RTS, DTR, OUT2
    }
}

pub fn print_byte_serial(byte: u8) {
    unsafe {
        while (inb(PORT_COM1 + 5) & 0x20) == 0 {}
        outb(PORT_COM1, byte);
    }
}

fn print_serial_str(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            print_byte_serial(b'\r');
        }
        print_byte_serial(b);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start")]
pub unsafe extern "sysv64" fn _start(_boot_info: &'static boot::BootInfo) -> ! {
    unsafe {
        // 1. Disabilita IMMEDIATAMENTE le interruzioni CPU
        core::arch::asm!("cli", options(nomem, nostack));

        // 2. Passa allo stack riservato del Kernel
        let stack_top = KERNEL_STACK.0.as_ptr() as u64 + 16384;
        core::arch::asm!("mov rsp, {}", in(reg) stack_top, options(nomem, nostack));

        // 3. Test Diagnostico
        outb(PORT_COM1, b'X');
        outb(PORT_COM1, b'\n');
    }

    // 4. Inizializzazione completa della seriale e stampa messaggio
    init_serial();
    print_serial_str("Welcome to Hugin Kernel!\n");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}