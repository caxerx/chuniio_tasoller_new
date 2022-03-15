use rusb::{DeviceHandle, GlobalContext};
use shared_memory::*;
use std::{thread::sleep, time::Duration};

fn init_usb(device: DeviceHandle<GlobalContext>) {
    let led_smem = create_led_shared_memory();
    let mut usb_out: [u8; 240] = [0; 240];

    let mut usb_in = [0u8; 36];

    let mut input_status_smem = create_input_shared_memory();

    loop {
        unsafe {
            let led_status = led_smem.as_slice();
            for i in 0..240 {
                usb_out[i] = led_status[i]
            }
        }

        device
            .write_bulk(0x03, &usb_out, Duration::from_millis(100))
            .expect("[chuniio] Error to write to tasoller");

        device
            .read_interrupt(0x84, usb_in[0..].as_mut(), Duration::from_millis(100))
            .expect("[chuniio] Failed to read data from tasoller");

        unsafe {
            let input_status_mut = input_status_smem.as_slice_mut();
            for (i, el) in usb_in.iter().enumerate() {
                input_status_mut[i] = *el;
            }
        }

        sleep(Duration::from_nanos(1_000_000));
    }
}

fn create_led_shared_memory() -> Shmem {
    let mut shmem = match ShmemConf::new().size(240).flink("tasoller_led").create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink("tasoller_led").open().unwrap(),
        Err(_e) => panic!("[tasoller-server] Failed to open shared memory"),
    };

    shmem.set_owner(true);

    unsafe {
        let usb_out = shmem.as_slice_mut();
        usb_out[0] = 0x42;
        usb_out[1] = 0x4C;
        usb_out[2] = 0x00;
    }

    return shmem;
}

fn create_input_shared_memory() -> Shmem {
    let mut shmem = match ShmemConf::new().size(36).flink("tasoller_input").create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink("tasoller_input").open().unwrap(),
        Err(_e) => panic!("[tasoller-server] Failed to open shared memory"),
    };

    shmem.set_owner(true);

    return shmem;
}

#[tokio::main]
async fn main() {
    std::fs::remove_file("tasoller_input").ok();
    std::fs::remove_file("tasoller_led").ok();

    println!("Tasoller USB Server Started");

    let mut device = match rusb::open_device_with_vid_pid(0x1ccf, 0x2333) {
        Some(dev) => dev,
        None => panic!("[chuniio] Cannot find tasoller"),
    };

    device
        .claim_interface(0)
        .expect("[chuniio] Unable to open tasoller");

    let blocking_task = tokio::task::spawn_blocking(|| {
        init_usb(device);
    });

    blocking_task.await.unwrap();
}
