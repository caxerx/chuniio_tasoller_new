use shared_memory::*;
use std::{sync::Arc, time::Duration};
use windows::core::HRESULT;

static mut LED_SHMEM: Option<Arc<Shmem>> = None;
static mut INPUT_SHMEM: Option<Arc<Shmem>> = None;

#[no_mangle]
pub extern "C" fn chuni_io_get_api_version() -> u16 {
    0x0101
}

fn create_led_shared_memory() -> Shmem {
    match ShmemConf::new().os_id("tasoller_led").open() {
        Ok(shmem) => shmem,
        Err(_) => {
            panic!("Failed to load shared memory: tasoller_led")
        }
    }
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
pub unsafe extern "C" fn chuni_io_slider_init() -> HRESULT {
    LED_SHMEM = Some(Arc::new(create_led_shared_memory()));
    INPUT_SHMEM = Some(Arc::new(create_input_shared_memory()));
    HRESULT(0)
}

#[no_mangle]
pub extern "C" fn chuni_io_slider_start(callback: unsafe extern "C" fn(data: *const u8)) {
    std::thread::spawn(move || loop {
        let input_shmem = unsafe {
            match &INPUT_SHMEM {
                None => {
                    continue;
                }
                Some(t) => t,
            }
        };

        unsafe {
            let mut report_status = [0u8; 32];
            let input = std::slice::from_raw_parts(input_shmem.as_ptr(), 36);
            for i in 0..32 {
                report_status[if i % 2 == 0 { 30 - i } else { 32 - i }] = input[i + 4];
            }
            callback(report_status.as_ptr());
        }

        std::thread::sleep(Duration::from_nanos(1_000_000))
    });
}

#[no_mangle]
pub extern "C" fn chuni_io_slider_set_leds(rgb: *const u8) {
    if rgb.is_null() {
        return;
    }

    let led_shmem = unsafe {
        match &LED_SHMEM {
            None => return,
            Some(t) => t,
        }
    };

    unsafe {
        let led_mut = std::slice::from_raw_parts_mut(led_shmem.as_ptr(), 240);
        let led_report = std::slice::from_raw_parts(rgb as *const u8, 96);

        for n in 0..31 {
            led_mut[n * 3 + 0 + 3] = led_report[n * 3 + 2];
            led_mut[n * 3 + 1 + 3] = led_report[n * 3 + 1];
            led_mut[n * 3 + 2 + 3] = led_report[n * 3 + 0];
        }
    }
}

#[no_mangle]
pub extern "C" fn chuni_io_slider_stop() {}

// ====== PLACEHOLDER ONLY ======
#[no_mangle]
pub extern "C" fn chuni_io_jvs_init() -> HRESULT {
    HRESULT(0)
}

#[no_mangle]
pub extern "C" fn chuni_io_jvs_poll(_opbtn: *mut u8, _beams: *mut u8) {}

#[no_mangle]
pub extern "C" fn chuni_io_jvs_read_coin_counter(_total: *mut u8) {}
