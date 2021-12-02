use crate::MacAddress;

use crate::eth::offset;

pub fn write_source_addr<'a>(dst: &mut [u8], addr: &MacAddress) {
    offset::ETH_SRC_ADDR.slice_mut(dst).copy_from_slice(&addr.0)
}

pub fn write_destination_addr(dst: &mut [u8], addr: &MacAddress) {
    offset::ETH_DST_ADDR.slice_mut(dst).copy_from_slice(&addr.0)
}

pub fn write_ether_type(dst: &mut [u8], ether_type: u16) {
    offset::ETH_ETHER_TYPE.slice_mut(dst).copy_from_slice(&ether_type.to_be_bytes())
}

pub fn slice_payload(src: &mut [u8], offset: usize, len: usize) -> &mut [u8] {
    &mut src[14 + offset..14 + offset + len]
}
