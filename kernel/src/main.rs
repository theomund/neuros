#![no_std]
#![no_main]

use core::arch::asm;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);
static BASE_REVISION: limine::BaseRevision = limine::BaseRevision::new(0);

#[no_mangle]
extern "C" fn _start() -> ! {
    assert!(BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            hcf();
        }

        let framebuffer = &framebuffer_response.framebuffers()[0];

        for i in 0..100_usize {
            let pixel_offset = i * framebuffer.pitch as usize + i * 4;
            let address = framebuffer
                .address
                .as_ptr()
                .unwrap()
                .wrapping_add(pixel_offset) as *mut u32;
            unsafe {
                *(address) = 0xFFFFFFFF;
            }
        }
    }

    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}
