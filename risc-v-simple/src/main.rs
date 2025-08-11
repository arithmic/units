#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Make the input dynamic by reading from volatile memory
    let n = unsafe { core::ptr::read_volatile(0x2000_0000 as *const u32) };
    // Fallback to 10 if the memory location is 0
    let input = if n == 0 { 10 } else { n % 20 }; // Cap at 20 to prevent overflow

    let result = fibonacci(input);

    // Demonstrate M extension usage
    let mul_result = multiply_m_extension(result, 2);
    let div_result = divide_m_extension(mul_result, 3);
    let mod_result = modulo_m_extension(div_result, 5);

    // Since this bare-metal program has no console/stdout, it writes results to specific memory addresses that the RISC-V simulator can monitor
    unsafe {
        core::ptr::write_volatile(0x1000_0000 as *mut u32, result);
        core::ptr::write_volatile(0x1000_0004 as *mut u32, mul_result);
        core::ptr::write_volatile(0x1000_0008 as *mut u32, div_result);
        core::ptr::write_volatile(0x1000_000C as *mut u32, mod_result);
    }

    loop {}
}

// Iterative Fibonacci implementation
#[inline(never)]
pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut a: u32 = 0;
    let mut b: u32 = 1;

    // Use black_box to prevent optimization
    for _ in 2..=core::hint::black_box(n) {
        let temp = a.wrapping_add(b);
        a = core::hint::black_box(b);
        b = core::hint::black_box(temp);
    }

    b
}

// Multiplication using RISC-V M extension
fn multiply_m_extension(a: u32, b: u32) -> u32 {
    // This will compile to the MUL instruction from M extension
    a.wrapping_mul(b)
}

// Division using RISC-V M extension
fn divide_m_extension(a: u32, b: u32) -> u32 {
    // This will compile to the DIV instruction from M extension
    if b != 0 {
        a / b
    } else {
        0
    }
}

// Modulo using RISC-V M extension
fn modulo_m_extension(a: u32, b: u32) -> u32 {
    // This will compile to the REM instruction from M extension
    if b != 0 {
        a % b
    } else {
        0
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
