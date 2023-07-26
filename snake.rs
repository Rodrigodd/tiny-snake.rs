//! Base setup gotten from: https://vulns.xyz/2023/03/linux-executable-from-scratch-with-x86_64-unknown-none-rust/
#![no_std]
#![no_main]

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("panic!");
    quit();
}

global_asm! {
    ".global _start",
    "_start:",
    // "xor rbp, rbp",
    // "movl edi, byte ptr [rsp]", // argc
    // "lea rsi, [rsp + 8]", // argv
    // "xor rax, rax",
    "call main",
}

fn exit(status: i32) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") status,
            options(noreturn)
        );
    }
}

unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 1isize => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 0isize => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
struct timespec {
    tv_sec: isize,
    tv_nsec: isize,
}

unsafe fn nanosleep(req: *const timespec, rem: *mut timespec) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 35isize => r0,
        in("rdi") req,
        in("rsi") rem,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

unsafe fn getrandom(buffer: *mut u8, size: usize, flags: core::ffi::c_uint) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 318isize => r0,
        in("rdi") buffer,
        in("rsi") size,
        in("rdx") flags,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

unsafe fn ioctl(fd: i32, request: u64, argp: *mut u8) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 16isize => r0,
        in("rdi") fd,
        in("rsi") request,
        in("rdx") argp,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
struct kernel_sigaction {
    sa_handler: unsafe extern "C" fn(i32),
    sa_flags: u64,
    sa_restorer: unsafe extern "C" fn(),
    sa_mask: u64,
}

unsafe fn rt_sigaction(
    sig: i32,
    act: *const kernel_sigaction,
    oact: *mut kernel_sigaction,
    sigsetsize: usize,
) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") 13isize => r0,
        in("rdi") sig,
        in("rsi") act,
        in("rdx") oact,
        in("r10") sigsetsize,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

extern "C" fn sig_handler(_sig: i32) {
    quit();
}

extern "C" {
    fn restorer();
}
global_asm! {
    // ".global restorer"
    "restorer:",
    "mov rax, 15",
    "syscall",
}

static HANDLER: kernel_sigaction = kernel_sigaction {
    sa_handler: sig_handler,
    sa_flags: 0x04000000,
    sa_restorer: restorer,
    sa_mask: 0,
};

fn register_sigint_handler() {
    unsafe {
        rt_sigaction(2, &HANDLER, core::ptr::null_mut(), 8);
    }
}

fn print(s: &str) {
    let mut s = s.as_bytes();
    unsafe {
        while s.len() > 0 {
            let n = write(1, s.as_ptr(), s.len());
            if n < 0 || n as usize > s.len() {
                print("write failed");
                exit(10);
            }
            s = &s[n as usize..];
        }
    }
}

fn getch() -> u8 {
    let mut c = 0;
    unsafe {
        loop {
            let n = read(0, &mut c as *mut u8, 1);
            if n < 0 {
                print("read failed");
                exit(11);
            }
            if n == 1 {
                break;
            }
        }
    }
    c
}

#[derive(Clone)]
#[allow(non_camel_case_types, dead_code)]
#[repr(C)]
struct kernel_termios {
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_line: u8,
    c_cc: [u8; 19],
}

static mut INITAL_TERM: kernel_termios = kernel_termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 19],
};

const STDIN: i32 = 0;
const FIONREAD: u64 = 0x541B;
const TCGETS: u64 = 0x5401;
const TCSETS: u64 = 0x5402;

fn enable_raw_mode() {
    const ICANON: u32 = 0x2;
    const ECHO: u32 = 0x8;

    let mut termios = kernel_termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 19],
    };
    unsafe {
        ioctl(
            STDIN,
            TCGETS,
            &mut termios as *mut kernel_termios as *mut u8,
        );
        INITAL_TERM = termios.clone();
        termios.c_lflag &= !ICANON;
        termios.c_lflag &= !ECHO;
        ioctl(
            STDIN,
            TCSETS,
            &mut termios as *mut kernel_termios as *mut u8,
        );
    }
}

fn disable_raw_mode() {
    unsafe {
        ioctl(
            STDIN,
            TCSETS,
            &mut INITAL_TERM as *mut kernel_termios as *mut u8,
        );
    }
}

fn _kbhit() -> bool {
    let mut n = 0;
    unsafe {
        ioctl(STDIN, FIONREAD, &mut n as *mut i32 as *mut u8);
    }
    n != 0
}

fn rand() -> u8 {
    let mut r = 0;
    unsafe {
        loop {
            let n = getrandom(&mut r as *mut u8, 1, 0);
            if n < 0 {
                print("getrandom failed");
                exit(12);
            }
            if n == 1 {
                break;
            }
        }
    }
    r
}

fn sleep(ms: u32) {
    unsafe {
        let mut req = timespec {
            tv_sec: ms as isize / 1000,
            tv_nsec: (ms as isize % 1000) * 1000000,
        };
        let mut rem = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        loop {
            let n = nanosleep(&req as *const timespec, &mut rem as *mut timespec);
            if n < 0 {
                print("nanosleep failed");
                exit(13);
            }
            if n == 0 {
                break;
            }
            req = rem;
        }
    }
}

fn quit() -> ! {
    // set cursor visible (`\x1b[?25h`), disable alternative screen (`\xb1[1049l`).
    print("\x1b[?25h\x1b[?1049l\x1b[2J\x1b[H");
    disable_raw_mode();

    exit(0);
}

fn win() -> ! {
    // set cursor visible (`\x1b[?25h`), disable alternative screen (`\xb1[1049l`).
    print("\x1b[?25h\x1b[?1049l\x1b[2J\x1b[H");
    disable_raw_mode();

    print("you won!\n");

    exit(0);
}

struct IntString {
    s: [u8; 10],
    start: usize,
}
impl core::ops::Deref for IntString {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.s.get_unchecked(self.start..)) }
    }
}

/// Convert an integer to a string.
fn int_to_string(mut n: u32) -> IntString {
    let mut s = [0; 10];
    let mut i = 9;
    loop {
        s[i] = (n % 10 + '0' as u32) as u8;
        n /= 10;
        i -= 1;
        if n == 0 || i == 0 {
            break;
        }
    }
    IntString { s, start: i }
}

fn clear_tile(pos: u8) {
    let x = pos as usize % W;
    let y = pos as usize / W;

    let x = int_to_string(x as u32 * 2 + 3);
    let y = int_to_string(y as u32 + 2);

    // set position (`\x1b[y;xH`), reset color (`\x1b[40m`), two spaces.
    print("\x1b[");
    print(&y);
    print(";");
    print(&x);
    print("H\x1b[0m  ");
}

fn draw_tile(color: u8, pos: u8) {
    let x = pos as usize % W;
    let y = pos as usize / W;

    // calling all int_to_string before all print, instead of interleaving them, is smaller.
    let y = int_to_string(y as u32 + 2);
    let x = int_to_string(x as u32 * 2 + 3);
    let color = int_to_string(40 + color as u32);

    // set position (`\x1b[y;xH`), set color (`\x1b[40m`), two spaces, reset color (`\x1b[0m`).
    print("\x1b[");
    print(&y);
    print(";");
    print(&x);
    print("H\x1b[");
    print(&color);
    print("m  \x1b[0m");
}

const W: usize = 16;
const H: usize = 16;

struct Snake {
    body: [u8; W * H],
    len: u8,
    apple: u8,
    dir: u8,
}
impl Snake {
    fn step(&mut self) {
        let mut head = self.body[0] as usize;
        match self.dir {
            0 => {
                if head % W == W - 1 {
                    quit();
                }
                head += 1
            }
            1 => {
                if head >= W * (H - 1) {
                    quit();
                }
                head += W
            }
            2 => {
                if head % W < 1 {
                    quit();
                }
                head -= 1
            }
            3 => {
                if head < W {
                    quit();
                }
                head -= W
            }
            _ => {}
        }

        let head = head as u8;

        for i in (0..self.len as usize).rev() {
            self.body[i + 1] = self.body[i];
            if head == self.body[i] {
                quit();
            }
        }
        self.body[0] = head;

        if head == self.apple {
            if self.len == 255 {
                win();
            }
            self.len += 1;
            clear_tile(self.apple);
            'a: loop {
                self.apple = rand();
                for i in 0..self.len as usize {
                    if self.apple == self.body[i] {
                        continue 'a;
                    }
                }
                break;
            }
            draw_tile(3, self.apple);
        } else {
            clear_tile(self.body[self.len as usize]);
        }

        draw_tile(2, head);
    }
}

fn draw_border() {
    print("\x1b[1;1H");
    print("\x1b[42m  ");
    for _ in 0..W {
        print("  ");
    }
    print("  \x1b[0m\n");
    for _ in 0..H {
        print("\x1b[42m  \x1b[0m");
        for _ in 0..W {
            print("  ");
        }
        print("\x1b[42m  \x1b[0m\n");
    }
    print("\x1b[42m  \x1b[0m");
    for _ in 0..W {
        print("\x1b[42m  \x1b[0m");
    }
    print("\x1b[42m  \x1b[0m\n");
}

#[no_mangle]
unsafe fn main() -> ! {
    // enter raw mode
    enable_raw_mode();

    register_sigint_handler();

    // set cursor invisible (`\x1b[?25l`), enable alternative screen (`\xb1[1049h`), clear screen (`\x1b[2J`), set cursor position (`\x1b[H`).
    print("\x1b[?25l\x1b[?1049h\x1b[2J\x1b[H");

    // draw border
    draw_border();

    // snake
    let mut snake = Snake {
        body: [0; W * H],
        len: 1,
        apple: 0,
        dir: 0,
    };
    snake.body[0] = W as u8 * (H as u8 / 2) + W as u8 / 2;
    loop {
        snake.apple = rand();
        if snake.apple != snake.body[0] {
            break;
        }
    }

    draw_tile(3, snake.apple);
    loop {
        snake.step();

        // delay
        sleep(300);

        // read arrows keys using getch
        let mut dir = snake.dir;
        while _kbhit() {
            let c = getch();
            if c == 27 {
                getch();
                match getch() {
                    // up
                    65 => dir = 3,
                    // down
                    66 => dir = 1,
                    // right
                    67 => dir = 0,
                    // left
                    68 => dir = 2,
                    _ => (),
                }
            }
        }
        if dir != (snake.dir + 2) % 4 {
            snake.dir = dir;
        }
    }

    // quit();
}
