fn main() {
    // udp: do not have function to devide data. so it has data size limit.
    // tcp: has divider to prevent ip layer from be pass too big data. Sequence number is from 0 when it is fulled.

    // &str: fixed length, reference to utf-8's byte sequence.
    // String: variable length, utf-8 is garanteed, heap allocated.
    let interface_name: &str = "interface name";
    let interfaces = pnet::datalink::interfaces();
    let network_interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("failed to get interface");
    // get datalink layer channels
    let (tx, mut rx) = match pnet::datalink::channel(&network_interface, Default::default()) {
        Ok(pnet::datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandle Channel"),
        Err(e) => panic!("failed to create channel {}", e),
    };

    loop {
        // construct ethernet frame from received data.
        match rx.next() {
            Ok(frame) => {
                let frame = pnet::packet::ethernet::EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    pnet::packet::ethernet::EtherTypes::Ipv4 => {}
                    pnet::packet::ethernet::EtherTypes::Ipv6 => {}
                    _ => {
                        print!("neither ipv4 nor ipv6 packet.")
                    }
                }
            }
            Err(e) => {
                panic!("failed to read {}", e)
            }
        };
    }
}
