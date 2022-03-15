use shared_memory::*;
use std::time::Duration;
use windows::core::HRESULT;

#[no_mangle]
pub extern "C" fn chuni_io_get_api_version() -> u16 {
    0x0101
}

fn create_led_shared_memory() -> Shmem {
    let shmem = ShmemConf::new().flink("tasoller_led").open().expect("");
    return shmem;
}

fn create_input_shared_memory() -> Shmem {
    let shmem = ShmemConf::new().flink("tasoller_input").open().expect("");
    return shmem;
}

#[no_mangle]
pub extern "C" fn chuni_io_slider_init() -> HRESULT {
    HRESULT(0)
}

#[no_mangle]
pub extern "C" fn chuni_io_slider_start(callback: unsafe extern "C" fn(data: *const u8)) {
    std::thread::spawn(move || loop {
        let mut report_status = [0u8; 32];
        let input_shmem = create_input_shared_memory();
        let mem_status = std::ptr::slice_from_raw_parts(input_shmem.as_ptr(), 36);

        unsafe {
            for i in 0..32 {
                report_status[if i % 2 == 0 { 30 - i } else { 32 - i }] = (*mem_status)[i + 4];
            }
        }

        unsafe {
            callback(report_status.as_ptr());
        }

        std::thread::sleep(Duration::from_nanos(1_000_000))
    });
}

#[no_mangle]
pub unsafe extern "C" fn chuni_io_slider_stop() {}

#[no_mangle]
pub extern "C" fn chuni_io_slider_set_leds(rgb: *const u8) {
    if rgb.is_null() {
        return;
    }

    let led_shmem = create_led_shared_memory();
    let led_mut = std::ptr::slice_from_raw_parts_mut(led_shmem.as_ptr(), 240);

    unsafe {
        let led_report = std::slice::from_raw_parts(rgb as *const u8, 96);

        for n in 0..31 {
            (*led_mut)[n * 3 + 0 + 3] = led_report[n * 3 + 2];
            (*led_mut)[n * 3 + 1 + 3] = led_report[n * 3 + 1];
            (*led_mut)[n * 3 + 2 + 3] = led_report[n * 3 + 0];
        }
    }
}

// ====== PLACEHOLDER ONLY ======
#[no_mangle]
pub extern "C" fn chuni_io_jvs_init() -> HRESULT {
    HRESULT(0)
}

#[no_mangle]
pub extern "C" fn chuni_io_jvs_poll(_opbtn: *mut u8, _beams: *mut u8) {}

#[no_mangle]
pub extern "C" fn chuni_io_jvs_read_coin_counter(_total: *mut u8) {}
