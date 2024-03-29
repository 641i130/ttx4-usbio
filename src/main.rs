use std::{time::Duration};

use rusb::{
    Context, Device, DeviceDescriptor, DeviceHandle, Direction, Result, TransferType, UsbContext,
};

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn main() {
    let vid: u16 = 0x0ae4;
    let pid: u16 = 0x0901;
    match Context::new() {
        Ok(mut context) => match open_device(&mut context, vid, pid) {
            Some((mut device, device_desc, mut handle)) => {
                read_device(&mut device, &device_desc, &mut handle).unwrap()
            }
            None => println!("Device {:04x}:{:04x} is not connected!", vid, pid),
        },
        Err(e) => panic!("Could not initialize libusb: {}", e),
    }
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

fn read_device<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
    handle: &mut DeviceHandle<T>,
) -> Result<()> {
    handle.reset()?;

    match find_readable_endpoint(device, device_desc, TransferType::Bulk) {
        Some(endpoint) => read_endpoint(handle, endpoint, TransferType::Bulk),
        None => println!("No readable bulk endpoint"),
    }

    Ok(())
}

fn find_readable_endpoint<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
    transfer_type: TransferType,
) -> Option<Endpoint> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    println!("Endpoint addr : {:?}",endpoint_desc.address());
                    if endpoint_desc.direction() == Direction::In
                        && endpoint_desc.transfer_type() == transfer_type
                    {
                        return Some(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                           address: endpoint_desc.address(),
                        });
                    }
                }
            }
        }
    }

    None
}

fn read_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: Endpoint,
    transfer_type: TransferType,
) {

    match configure_endpoint(handle, &endpoint) {
        Ok(_) => {
            println!("Taito USBIO Monitor");
            println!("Values are in base 10!!!");
            println!("");
            let mut buf = [0; 64]; // Buffer size!!!
            let timeout = Duration::from_secs(1); // Check every second?
            match transfer_type {
                TransferType::Bulk => match handle.read_bulk(endpoint.address, &mut buf, timeout) {
                    Ok(len) => {
                        // TODO Parse buffer and map where each button is in relation to the code
                        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}","Index","Byte Val","Index","Byte Val");
                        // &buf[]
                        let mut i = 0;
                        loop {
                            if i >= len {
                                break;
                            }
                            println!("{0: <10} | {1: <10?} | {2: <10} | {3: <10?}", &i, &buf[i], &i+1, &buf[i+1]);
                            i+=2;
                        }
                    }
                    Err(err) => println!("could not read from endpoint: {:?}", err),
                },
                _ => (),
            }
        }
        Err(err) => println!("could not configure endpoint: {:?}", err),
    }
}

fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<()> {
    handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;
    Ok(())
}
