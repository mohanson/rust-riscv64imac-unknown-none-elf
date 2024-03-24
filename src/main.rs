#![no_main]
#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

static mut HEAPS: [u8; 1024] = [0; 1024];
#[global_allocator]
static ALLOC: linked_list_allocator::LockedHeap = linked_list_allocator::LockedHeap::empty();

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    // If the main thread panics it will terminate all your threads and end your program with code 101.
    // See: https://github.com/rust-lang/rust/blob/master/library/core/src/macros/panic.md
    syscall_exit(101)
}

fn syscall(mut a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64) -> u64 {
    unsafe {
        core::arch::asm!(
          "ecall",
          inout("a0") a0,
          in("a1") a1,
          in("a2") a2,
          in("a3") a3,
          in("a4") a4,
          in("a5") a5,
          in("a6") a6,
          in("a7") a7
        )
    }
    a0
}

fn syscall_exit(code: u64) -> ! {
    syscall(code, 0, 0, 0, 0, 0, 0, 93);
    loop {}
}

fn syscall_write(buf: *const u8) -> u64 {
    syscall(buf as u64, 0, 0, 0, 0, 0, 0, 2177)
}

#[no_mangle]
pub unsafe extern "C" fn _start() {
    core::arch::asm!(
        "lw a0,0(sp)", // Argc.
        "add a1,sp,8", // Argv.
        "li a2,0",     // Envp.
        "call main",
        "li a7, 93",
        "ecall",
    );
}

#[no_mangle]
unsafe extern "C" fn main(argc: u64, argv: *const *const i8) -> u64 {
    unsafe {
        ALLOC.lock().init(HEAPS.as_mut_ptr(), 1024);
    }
    let mut args = Vec::new();
    for i in 1..argc {
        let argn = core::ffi::CStr::from_ptr(argv.add(i as usize).read());
        let argn = String::from(argn.to_string_lossy());
        args.push(argn);
    }
    let data = args.join(" ");
    if data.is_empty() {
        syscall_write(String::from("Hello World!").as_ptr() as *const u8);
    } else {
        syscall_write(data.as_ptr());
    }
    return 0;
}
