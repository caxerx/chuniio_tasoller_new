use rusb::{DeviceHandle, GlobalContext};
use shared_memory::*;
use std::{thread::sleep, time::Duration};

async fn init_usb(device: DeviceHandle<GlobalContext>) {
    let led_smem = create_led_shared_memory();
    let mut usb_out: [u8; 240] = [0; 240];

    let mut usb_in = [0u8; 36];

    let mut input_status_smem = create_input_shared_memory();

    loop {
        // Map led status from shared memory
        unsafe {
            let led_status = led_smem.as_slice();
            for i in 0..240 {
                usb_out[i] = led_status[i]
            }
        }

        // Write led status to usb
        match device.write_bulk(0x03, &usb_out, Duration::from_micros(1)) {
            Ok(_) => (),
            Err(e) => println!("Error when write to tasoller {}", e),
        }

        // Read input status from tasoller
        match device.read_interrupt(0x84, usb_in[0..].as_mut(), Duration::from_micros(1)) {
            Ok(_) => (),
            Err(e) => println!("Failed to read data from tasoller {}", e),
        }

        // Write pressure status to shared memory
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
    let mut shmem = match ShmemConf::new().size(240).os_id("tasoller_led").create() {
        Ok(m) => m,
        Err(ShmemError::MappingIdExists) => ShmemConf::new().os_id("tasoller_led").open().unwrap(),
        Err(_) => {
            println!("Failed to create shared memory: tasoller_led");
            panic!()
        }
    };

    shmem.set_owner(true);

    unsafe {
        let usb_out = shmem.as_slice_mut();
        usb_out[0] = 0x42;
        usb_out[1] = 0x4C;
        usb_out[2] = 0x00;
        for i in 3..240 {
            usb_out[i] = 0x00;
        }
    }

    return shmem;
}

fn create_input_shared_memory() -> Shmem {
    let mut shmem = match ShmemConf::new().size(36).os_id("tasoller_input").create() {
        Ok(m) => m,
        Err(ShmemError::MappingIdExists) => {
            ShmemConf::new().os_id("tasoller_input").open().unwrap()
        }
        Err(_) => {
            println!("Failed to create shared memory: tasoller_input");
            panic!()
        }
    };

    shmem.set_owner(true);

    return shmem;
}

#[tokio::main]
async fn main() {
    println!("Tasoller Server Started");

    let mut device = match rusb::open_device_with_vid_pid(0x1ccf, 0x2333) {
        Some(dev) => dev,
        None => {
            println!("Cannot find tasoller");
            panic!()
        }
    };

    match device.claim_interface(0) {
        Ok(_) => (),
        Err(_) => {
            println!("Cannot open tasoller");
            panic!()
        }
    }

    let task = tokio::spawn(init_usb(device));

    task.await.ok();
}
