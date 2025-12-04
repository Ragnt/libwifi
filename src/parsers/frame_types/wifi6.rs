use nom::bytes::complete::take;
use nom::sequence::tuple;

use crate::error::Error;
use crate::frame::components::FrameControl;
use crate::frame::{Frame, NdpAnnouncement, Trigger};
use crate::parsers::components::{clone_slice, parse_mac};

/// Parse a WiFi 6 [Trigger] frame.
///
/// The general structure is:
/// - FrameControl
/// - Duration
/// - Receiver Address (RA)
/// - Common Info and User Info fields (stored as raw_data for now)
pub fn parse_trigger(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (remaining, (duration, receiver_address)) = tuple((take(2usize), parse_mac))(input)?;

    Ok(Frame::Trigger(Trigger {
        frame_control,
        duration: clone_slice::<2>(duration),
        receiver_address,
        raw_data: remaining.to_vec(),
    }))
}

/// Parse a WiFi 6 [NdpAnnouncement] frame.
///
/// The general structure is:
/// - FrameControl
/// - Duration
/// - Receiver Address (RA)
/// - Additional sounding parameters (stored as raw_data for now)
pub fn parse_ndp_announcement(frame_control: FrameControl, input: &[u8]) -> Result<Frame, Error> {
    let (remaining, (duration, receiver_address)) = tuple((take(2usize), parse_mac))(input)?;

    Ok(Frame::NdpAnnouncement(NdpAnnouncement {
        frame_control,
        duration: clone_slice::<2>(duration),
        receiver_address,
        raw_data: remaining.to_vec(),
    }))
}
