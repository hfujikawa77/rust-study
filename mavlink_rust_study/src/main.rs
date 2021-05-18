use std::{thread, time::Duration};

fn main() {
    println!("1");
    let mut mavconn =
        mavlink::connect::<mavlink::ardupilotmega::MavMessage>("udpout:127.0.0.1:14551").unwrap();

    println!("2");
    mavconn.set_protocol_version(mavlink::MavlinkVersion::V1);
    
    println!("3");
    let vehicle = std::sync::Arc::new(mavconn);
    
    println!("4");
    thread::spawn({
        let vehicle = vehicle.clone();
        move || loop {
            let res = vehicle.send_default(&heartbeat_message().into());
            if res.is_ok() {
                thread::sleep(Duration::from_secs(1));
            } else {
                println!("send failed: {:?}", res);
            }
        }
    });

    loop {
        match vehicle.recv() {
            Ok((_header, msg)) => {
                println!("received: {:?}", msg);
            }
            Err(error) => {
                println!("recv error: {:?}", error);
                break;
            }
        }
    }
 
}

pub fn heartbeat_message() -> mavlink::common::MavMessage {
    mavlink::common::MavMessage::HEARTBEAT(mavlink::common::HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: mavlink::common::MavType::MAV_TYPE_SUBMARINE,
        autopilot: mavlink::common::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: mavlink::common::MavModeFlag::empty(),
        system_status: mavlink::common::MavState::MAV_STATE_STANDBY,
        mavlink_version: 0x3,
    })
}
