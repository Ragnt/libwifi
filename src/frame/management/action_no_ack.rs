use libwifi_macros::AddressHeader;
use crate::frame::components::*;

/// Action frame without acknowledgement requirement (WiFi 6)
#[derive(Clone, Debug, AddressHeader)]
pub struct ActionNoAck {
    pub header: ManagementHeader,
    pub category: u8,
    pub action: u8,
    pub station_info: StationInfo,
}

impl ActionNoAck {
    pub fn encode(&self) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();

        // Encode the ManagementHeader
        encoded.extend(self.header.encode());

        // Encode the category and action
        encoded.push(self.category);
        encoded.push(self.action);

        // Encode StationInfo if necessary
        encoded.extend(self.station_info.encode());

        encoded
    }
}
