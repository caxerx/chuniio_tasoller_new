use std::sync::Arc;

use shared_memory::*;
use windows::core::HRESULT;

static mut CHUNI_IO_COINS: u16 = 0;
static mut CHUNI_IO_COIN: bool = false;
static mut INPUT_SHMEM: Option<Arc<Shmem>> = None;

#[no_mangle]
pub extern "C" fn chuni_io_get_api_version() -> u16 {
    0x0101
}

fn create_input_shared_memory() -> Shmem {
    match ShmemConf::new().os_id("tasoller_input").open() {
        Ok(shmem) => shmem,
        Err(_) => {
            panic!("Failed to load shared memory: tasoller_input")
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn chuni_io_jvs_init() -> HRESULT {
    INPUT_SHMEM = Some(Arc::new(create_input_shared_memory()));
    HRESULT(0)
}

#[no_mangle]
pub extern "C" fn chuni_io_jvs_poll(opbtn: *mut u8, beams: *mut u8) {
    let input_shmem = unsafe {
        match &INPUT_SHMEM {
            None => return,
            Some(t) => t,
        }
    };

    unsafe {
        let input = std::slice::from_raw_parts(input_shmem.as_ptr(), 36);
        let bit = input[3];
        // fn1
        if bit & (1 << 6) != 0 {
            *opbtn |= 0x1;
        }

        // fn2
        if bit & (1 << 7) != 0 {
            *opbtn |= 0x2;
        }

        for i in 0..6 {
            if bit & (1 << i) != 0 {
                *beams |= 1 << i;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn chuni_io_jvs_read_coin_counter(out: *mut u16) {
    if out.is_null() {
        return;
    }

    let input_shmem = unsafe {
        match &INPUT_SHMEM {
            None => return,
            Some(t) => t,
        }
    };

    unsafe {
        let input = std::slice::from_raw_parts(input_shmem.as_ptr(), 36);
        let bit = input[3];
        // fn1
        if bit & (1 << 6) != 0 {
            if !CHUNI_IO_COIN {
                CHUNI_IO_COIN = true;
                CHUNI_IO_COINS += 1;
            } else {
                CHUNI_IO_COIN = false;
            }
        }

        *out = CHUNI_IO_COINS
    }
}

// ======== PLACEHOLDER ONLY =======
#[no_mangle]
pub extern "C" fn chuni_io_slider_init() -> HRESULT {
    HRESULT(0)
}

#[no_mangle]
pub extern "C" fn chuni_io_slider_start() {}

#[no_mangle]
pub extern "C" fn chuni_io_slider_stop() {}

#[no_mangle]
pub extern "C" fn chuni_io_slider_set_leds(_rgb: *mut u8) {}
