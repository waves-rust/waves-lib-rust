use base58::*;

pub (crate) struct Buffer {
    buf: Vec<u8>
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { buf: Vec::new() }
    }

    pub fn from_bytes(b: &[u8]) -> Buffer {
        Buffer { buf: Vec::from(b) }
    }

    pub fn bytes(self: &mut Buffer, b: &[u8]) -> &mut Buffer {
        self.buf.extend_from_slice(b);
        self
    }

    pub fn byte(self: &mut Buffer, b: u8) -> &mut Buffer {
        self.buf.push(b);
        self
    }

    pub fn size(&mut self, n: usize) -> &mut Buffer {
        let bytes = [((n >> 8) & 0xff) as u8, (n & 0xff) as u8];
        self.bytes(&bytes)
    }

    pub fn long(&mut self, n: u64) -> &mut Buffer {
        let bytes = [
            ((n >> 56) & 0xff) as u8, ((n >> 48) & 0xff) as u8,
            ((n >> 40) & 0xff) as u8, ((n >> 32) & 0xff) as u8,
            ((n >> 24) & 0xff) as u8, ((n >> 16) & 0xff) as u8,
            ((n >> 8) & 0xff) as u8, (n & 0xff) as u8];
        self.bytes(&bytes)
    }

    pub fn boolean(&mut self, b: bool) -> &mut Buffer {
        let val = if b {1} else {0};
        self.buf.push(val);
        self
    }

    pub fn recipient(&mut self, chain_id: u8, recipient: &str) -> &mut Buffer {
        if recipient.len() <= 30 {
            // assume an alias
            self.byte(0x02).byte(chain_id).size(recipient.len()).bytes(&recipient.as_bytes())
        } else {
            self.bytes(&recipient.from_base58().unwrap().as_slice())
        }
    }

    pub fn array(&mut self, arr: &[u8]) -> &mut Buffer {
        self.size(arr.len()).bytes(arr)
    }

    pub fn asset_id(&mut self, asset_id: Option<&[u8]>) -> &mut Buffer {
        match asset_id {
            Some(id) => self.byte(1).bytes(id),
            None => self.byte(0)
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buf.as_slice()
    }
}
