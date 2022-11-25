use std::mem::size_of;

use libc::{c_void, AF_PACKET, ETH_P_ALL, IFNAMSIZ, SIOCGIFINDEX, SOCK_RAW};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    let socket = unsafe { libc::socket(AF_PACKET, SOCK_RAW, (ETH_P_ALL as u16).to_be() as i32) };
    if socket < 0 {
        let message: *const str = "no socket available";
        unsafe { libc::perror(message as *const i8) };
        panic!()
    };

    let interface_name = "lo";
    let mut interface = Interface {
        name: {
            let mut name = [0u8; IFNAMSIZ];
            name[..interface_name.as_bytes().len()].copy_from_slice(interface_name.as_bytes());
            name
        },
        index: 0,
    };

    let interface_pointer: *mut Interface = &mut interface;
    // SIOCG = Socket I/O Configuration Get
    // IF = interface
    if unsafe { libc::ioctl(socket, SIOCGIFINDEX, interface_pointer) } != 0 {
        let message: *const str = "no interface";
        unsafe { libc::perror(message as *const i8) };
        panic!()
    }
    info!("interface index: {}", interface.index);

    let socket_address = LowLevelSocketAddress {
        family: AF_PACKET as u16,
        protocol: (ETH_P_ALL as u16).to_be(),
        interface_index: interface.index,
        hardware_type: 0,
        packet_type: 0,
        hardware_address_length: 0,
        hardware_address: [0; 8],
    };
    let socket_address_pointer: *const LowLevelSocketAddress = &socket_address;
    if unsafe {
        libc::bind(
            socket,
            socket_address_pointer.cast::<libc::sockaddr>(),
            size_of::<libc::sockaddr_ll>() as u32,
        )
    } != 0
    {
        let message: *const str = "bind failed";
        unsafe { libc::perror(message as *const i8) };
        panic!()
    }

    const DATA_LENGTH: usize = 1024;
    loop {
        let data = &mut [0u8; DATA_LENGTH];
        let (ethernet_data, packet_length) = {
            let length = unsafe {
                libc::recv(
                    socket,
                    (data as *mut [u8]).cast::<c_void>(),
                    DATA_LENGTH,
                    0, /* no flags */
                )
            };
            (*data, length)
        };
        if packet_length <= 0 {
            let message: *const str = "recv failed";
            unsafe { libc::perror(message as *const i8) };
            panic!()
        }
        let ethernet_header = (&ethernet_data as *const [u8]).cast::<EthernetHeader>();
        let ethrnet_type = unsafe { (*ethernet_header).eth_type }.to_le();
        if ethrnet_type != ETHERNET_TYPE_IPV4 {
            continue;
        }
        let ip_data = &ethernet_data[size_of::<EthernetHeader>()..];
        let ip_header = (ip_data as *const [u8]).cast::<InternetHeader>();
        let next_protocol_type = unsafe { (*ip_header).protocol };
        if next_protocol_type == 1 {
            info!("received ping!");
            info!("{:?}", unsafe { &(*ip_header) });
            info!("ICMP? {:?}", &ip_data[size_of::<EthernetHeader>()..]);
        }
    }
}

#[repr(C)]
struct Interface {
    name: [u8; IFNAMSIZ],
    index: i32,
}

#[repr(C)]
struct LowLevelSocketAddress {
    /// Always AF_PACKET
    family: u16,
    protocol: u16,
    interface_index: i32,
    hardware_type: u16,
    packet_type: u8,
    hardware_address_length: u8,
    hardware_address: [u8; 8],
}

const ETHERNET_ADDRESS: usize = 6;
#[repr(C)]
struct EthernetHeader {
    destination_host: [u8; ETHERNET_ADDRESS],
    source_host: [u8; ETHERNET_ADDRESS],
    // https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
    eth_type: u16,
}
const ETHERNET_TYPE_IPV4: u16 = 0x08;

#[derive(Debug)]
#[repr(C)]
struct InternetHeader {
    version_and_header_length: u8,
    type_of_service: u8,
    total_length: u16,
    identification: u16,
    flags_and_fragment_offset: u16,
    time_to_live: u8,
    protocol: u8,
    header_chech_sum: u16,
    source_address: [u8; 4],
    destination_address: [u8; 4],
}
