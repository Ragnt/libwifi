use crate::frame::components::{FrameControl, MacAddress};
use crate::Addresses;

/// WiFi 6 (802.11ax) Trigger frame
/// Used for uplink multi-user transmissions
#[derive(Clone, Debug)]
pub struct Trigger {
    pub frame_control: FrameControl,
    pub duration: [u8; 2],
    pub receiver_address: MacAddress,
    pub raw_data: Vec<u8>,
}

impl Addresses for Trigger {
    fn src(&self) -> Option<&MacAddress> {
        None
    }

    fn dest(&self) -> &MacAddress {
        &self.receiver_address
    }

    fn bssid(&self) -> Option<&MacAddress> {
        None
    }
}

/// WiFi 6 (802.11ax) NDP Announcement frame
/// Null Data Packet announcement for sounding
#[derive(Clone, Debug)]
pub struct NdpAnnouncement {
    pub frame_control: FrameControl,
    pub duration: [u8; 2],
    pub receiver_address: MacAddress,
    pub raw_data: Vec<u8>,
}

impl Addresses for NdpAnnouncement {
    fn src(&self) -> Option<&MacAddress> {
        None
    }

    fn dest(&self) -> &MacAddress {
        &self.receiver_address
    }

    fn bssid(&self) -> Option<&MacAddress> {
        None
    }
}
