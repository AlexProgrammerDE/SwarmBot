use bytes::{BytesMut, BufMut};

pub struct ByteWriter {
    bytes: BytesMut,
}

impl ByteWritable for u8 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_u8(self);
    }
}


impl ByteWritable for bool {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        let val: u8 = if self {1} else {0};
        writer.write(val);
    }
}

impl ByteWritable for u128 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_u128(self);
    }
}

impl ByteWritable for i16 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_i16(self);
    }
}

impl ByteWritable for f64 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_f64(self);
    }
}

impl ByteWritable for u64 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_u64(self);
    }
}

impl ByteWritable for f32 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_f32(self);
    }
}

impl ByteWritable for u16 {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.bytes.put_u16(self);
    }
}

pub trait ByteWritable {
    fn write_to_bytes(self, writer: &mut ByteWriter);
}

pub trait ByteWritableLike {
    type Param;
    fn write_to_bytes_like(self, writer: &mut ByteWriter, param: &Self::Param);
}

impl ByteWriter {
    pub fn write<T: ByteWritable>(&mut self, value: T) -> &mut Self {
        value.write_to_bytes(self);
        self
    }

    pub fn write_like<T: ByteWritableLike<Param = P>, P>(&mut self, value: T, param: &P) -> &mut Self {
        value.write_to_bytes_like(self, param);
        self
    }

    pub fn new() -> Self {
        ByteWriter {
            bytes: BytesMut::new()
        }
    }

    pub fn freeze(self) -> Vec<u8> {
        self.bytes.freeze().to_vec()
    }

}