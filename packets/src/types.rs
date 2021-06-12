use crate::read::ByteReadable;
use crate::read::ByteReadableLike;
use crate::read::ByteReader;
use crate::write::ByteWritable;
use crate::write::ByteWritableLike;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use crate::write::ByteWriter;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};

pub trait Packet {
    const ID: u32;
    const STATE: PacketState;
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PacketState {
    Handshake,
    Status,
    Login,
    Play,
}

#[derive(Copy, Clone, Debug)]
pub struct VarInt(pub i32);

pub type Angle = u8;
pub type Identifier = String;
pub type Chat = String;

pub struct BitField {
    pub values: [bool; 8],
}
impl From<u8> for BitField {
    fn from(mut byte: u8) -> BitField {
        let mut values = [false; 8];
        let mut i = 0;
        while byte != 0 {
            let val = byte & 0b10000000 != 0;
            values[i] = val;
            byte <<= 1;
            i += 1;
        }
        BitField {
            values
        }
    }

}


#[derive(Copy, Clone)]
pub struct VarUInt(pub usize);

impl From<i32> for VarInt {
    fn from(input: i32) -> Self {
        VarInt(input)
    }
}

/// Writes like a Vec but without len
#[derive(Debug)]
pub struct RawVec<T = u8>(pub Vec<T>);

impl<T> RawVec<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // TODO: how does ownership inference work here
    #[inline]
    pub fn inner(self) -> Vec<T> {
        self.0
    }
}

impl ByteReadable for RawVec {
    fn read_from_bytes(byte_reader: &mut ByteReader) -> Self {
        let mut inner = Vec::new();
        while !byte_reader.empty() {
            let value: u8 = byte_reader.read();
            inner.push(value);
        }
        RawVec(inner)
    }
}

impl From<Vec<u8>> for RawVec {
    fn from(data: Vec<u8>) -> Self {
        RawVec(data)
    }
}

impl ByteWritable for RawVec {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        for value in self.inner() {
            writer.write(value);
        }
    }
}

impl<T: ByteReadable> ByteReadableLike for RawVec<T> {
    type Param = usize;

    fn read_from_bytes(byte_reader: &mut ByteReader, param: &usize) -> Self {
        let len = *param;
        let mut inner: Vec<T> = Vec::with_capacity(len);
        for _ in 0..len {
            inner.push(byte_reader.read());
        }
        RawVec(inner)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UUIDHyphenated(pub u128);

impl ByteReadable for UUIDHyphenated {
    fn read_from_bytes(byte_reader: &mut ByteReader) -> Self {
        let mut str: String = byte_reader.read();
        str = str.replace("-", "");
        UUIDHyphenated(u128::from_str_radix(&str, 16).unwrap())
    }
}

impl From<UUIDHyphenated> for UUID {
    fn from(hyph: UUIDHyphenated) -> Self {
        UUID(hyph.0)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct UUID(pub u128);

impl From<&String> for UUID {
    fn from(s: &String) -> Self {
        let inner = u128::from_str_radix(s, 16).unwrap();
        UUID(inner)
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:032x}", self.0))
    }
}

impl ByteWritable for UUID {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        writer.write(self.0);
    }
}

impl ByteReadable for UUID {
    fn read_from_bytes(byte_reader: &mut ByteReader) -> Self {
        let inner: u128 = byte_reader.read();
        UUID(inner)
    }
}

impl From<usize> for VarInt {
    fn from(input: usize) -> Self {
        VarInt(input as i32)
    }
}

impl From<u32> for VarInt {
    fn from(input: u32) -> Self {
        VarInt(input as i32)
    }
}

// pub type NBT = nbt::Blob;


/// total of 8 bytes
/// [See](https://wiki.vg/index.php?title=Protocol&oldid=14204#Position)
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}


// TODO: is this right
impl ByteReadable for Position {
    ///
    fn read_from_bytes(byte_reader: &mut ByteReader) -> Self {
        let val: u64 = byte_reader.read();


        let mut x = (val >> 38) as i32;
        let mut y = ((val >> 26) & 0xFFF) as i16;
        let mut z = (val << 38 >> 38) as i32;

        const LAT_LON_THRESHOLD: i32 = 1 << 25;
        const LAT_LON_SUB: i32 = 1 << 26;

        const Y_THRESH: i16 = 1 << 11;
        const Y_SUB: i16 = 1 << 12;

        if x >= LAT_LON_THRESHOLD { x -= LAT_LON_SUB }
        if y >= Y_THRESH { y -= Y_SUB }
        if z >= LAT_LON_THRESHOLD { z -= LAT_LON_SUB }

        Position {
            x,
            y,
            z,
        }
    }
}

impl ByteWritable for VarInt {
    fn write_to_bytes(self, writer: &mut ByteWriter) {
        const PART: u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                writer.write(val as u8);
                return;
            }
            writer.write(((val & PART) | 0x80) as u8);
            val >>= 7;
        }
    }
}

impl ByteReadable for VarInt {
    fn read_from_bytes(byte_reader: &mut ByteReader) -> Self {
        const PART: u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;
        loop {
            let b: u8 = byte_reader.read();
            let b = b as u32;
            val |= (b & PART) << (size * 7);
            size += 1;
            if size > 5 {
                panic!("oop");
            }
            if (b & 0x80) == 0 {
                break;
            }
        }
        VarInt(val as i32)
    }
}

impl ByteReadable for VarUInt {
    fn read_from_bytes(byte_reader: &mut ByteReader) -> Self {
        let VarInt(contents) = byte_reader.read();
        VarUInt(contents as usize)
    }
}