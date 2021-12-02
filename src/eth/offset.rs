pub(super) struct Offset(usize, usize);

impl Offset {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn len(&self) -> usize {
        self.1
    }

    pub fn slice<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.0..self.0+self.1]
    }

    pub fn slice_mut<'a>(&self, bytes: &'a mut [u8]) -> &'a mut [u8] {
        &mut bytes[self.0..self.0+self.1]
    }
}

pub(super) const ETH_DST_ADDR: Offset = Offset(0, 6);
pub(super) const ETH_SRC_ADDR: Offset = Offset(6, 6);

pub(super) const ETH_ETHER_TYPE: Offset = Offset(12, 2);
