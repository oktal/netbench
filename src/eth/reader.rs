use crate::eth::offset;

pub fn read_ether_type(bytes: &[u8]) -> Option<u16> {
    Some(
        u16::from_be_bytes(
            offset::ETH_ETHER_TYPE.slice(bytes).try_into().ok()?
        )
    )
}

pub fn slice_payload(src: &[u8], offset: usize, len: usize) -> &[u8] {
    &src[14 + offset..14 + offset + len]
}
