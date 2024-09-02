#![no_std]
#![no_main]

use psp::sys;
use psp::{BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};

psp::module!("sample_module", 1, 1);

static mut VRAM: *mut u32 = 0x4000_0000 as *mut u32;

fn psp_main() {
    psp::enable_home_button();
    unsafe {
        sys::sceDisplaySetMode(
            sys::DisplayMode::Lcd,
            SCREEN_WIDTH as usize,
            SCREEN_HEIGHT as usize,
        );

        // Cache-through address
        VRAM = (0x4000_0000u32 | sys::sceGeEdramGetAddr() as u32) as *mut u32;

        sys::sceDisplaySetFrameBuf(
            VRAM as *const u8,
            BUF_WIDTH as usize,
            sys::DisplayPixelFormat::Psm8888,
            sys::DisplaySetBufSync::NextFrame,
        );

        loop {
            sys::sceDisplayWaitVblankStart();
            for pos in 0..150 {
                let color = wheel(pos);

                for i in 0..(BUF_WIDTH * SCREEN_HEIGHT) {
                    *VRAM.add(i as usize) = color;
                }
            }
        }
    }
}

fn wheel(mut pos: u8) -> u32 {
    pos = 150 - pos;
    if pos < 85 {
        u32::from_be_bytes([150 - pos * 3, 0, pos * 3, 150])
    } else if pos < 130 {
        pos -= 85;
        u32::from_be_bytes([0, pos * 3, 150 - pos * 3, 150])
    } else {
        pos -= 130;
        u32::from_be_bytes([pos * 3, 150 - pos * 3, 0, 150])
    }
}
