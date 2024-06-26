use std::io::{Write, Read};

use anyhow::{bail, Context};
use shrinkwraprs::Shrinkwrap;


pub trait Encode {
    fn encode_into<S: Write>(&self, stream: &mut S) -> anyhow::Result<()>;
}


pub trait ConstSizedEncode: Sized {
    const ENCODED_SIZE: usize;
}

pub trait DynSizedEncode {
    fn encoded_size(&self) -> usize;
}

impl<T> DynSizedEncode for T
where T: ConstSizedEncode {
    fn encoded_size(&self) -> usize {
        Self::ENCODED_SIZE
    }
}


pub trait EncodingTag {
    const ENCODING_TAG: [u8; 2];
}


pub trait Decode: Sized {
    fn decode_from<S: Read>(stream: &mut S) -> anyhow::Result<Self>;
}




impl Encode for i8 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl ConstSizedEncode for i8 {
    const ENCODED_SIZE: usize = 1;
}

impl EncodingTag for i8 {
    const ENCODING_TAG: [u8; 2] = [0x81, 0x08];
}

impl Decode for i8 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 1];
        stream.read_exact(&mut buf).context("Failed to read bytes from stream for i8")?;
        let val = i8::from_be_bytes(buf);
        Ok(val)
    }
}



impl Encode for i16 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl ConstSizedEncode for i16 {
    const ENCODED_SIZE: usize = 2;
}

impl EncodingTag for i16 {
    const ENCODING_TAG: [u8; 2] = [0x81, 0x16];
}

impl Decode for i16 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).context("Failed to read bytes from stream for i16")?;
        let val = i16::from_be_bytes(buf);
        Ok(val)
    }
}



impl Encode for u16 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Decode for u16 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).context("Failed to read bytes from stream for u16")?;
        let val = u16::from_be_bytes(buf);
        Ok(val)
    }
}



impl Encode for i32 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl ConstSizedEncode for i32 {
    const ENCODED_SIZE: usize = 4;
}

impl EncodingTag for i32 {
    const ENCODING_TAG: [u8; 2] = [0x81, 0x32];
}

impl Decode for i32 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).context("Failed to read bytes from stream for i32")?;
        let val = i32::from_be_bytes(buf);
        Ok(val)
    }
}



impl Encode for u32 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl ConstSizedEncode for u32 {
    const ENCODED_SIZE: usize = 4;
}

impl EncodingTag for u32 {
    const ENCODING_TAG: [u8; 2] = [0x71, 0x32];
}

impl Decode for u32 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).context("Failed to read bytes from stream for u32")?;
        let val = u32::from_be_bytes(buf);
        Ok(val)
    }
}



impl Encode for i64 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl ConstSizedEncode for i64 {
    const ENCODED_SIZE: usize = 8;
}

impl EncodingTag for i64 {
    const ENCODING_TAG: [u8; 2] = [0x81, 0x64];
}

impl Decode for i64 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf).context("Failed to read bytes from stream for i64")?;
        let val = i64::from_be_bytes(buf);
        Ok(val)
    }
}



#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringUtf8(pub String);

impl StringUtf8 {
    #[inline]
    pub fn len_tagged(&self) -> Tagged<i16> {
        Tagged::new(self.len() as i16)
    }

    #[inline]
    pub fn bytes_encoded(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Encode for StringUtf8 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        self.len_tagged().encode_into(stream)?;
        stream.write_all(&self.bytes_encoded())?;
        Ok(())
    }
}

impl DynSizedEncode for StringUtf8 {
    #[inline]
    fn encoded_size(&self) -> usize {
        let len = self.len_tagged();
        len.encoded_size() + len.into_inner() as usize
    }
}

impl EncodingTag for StringUtf8 {
    const ENCODING_TAG: [u8; 2] = [0xAC, 0x08];
}

impl Decode for StringUtf8 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let len_tagged = Tagged::<i16>::decode_from(stream).context("Failed to decode UTF8 string length")?;
        let len = len_tagged.into_inner() as usize;

        let mut content_buf = vec![0u8; len];
        stream.read_exact(&mut content_buf).context("Failed to read UTF8 string contents")?;

        let s = String::from_utf8_lossy(&content_buf).to_string();
        Ok(Self(s))
    }
}

impl std::fmt::Display for StringUtf8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for StringUtf8 {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for StringUtf8 {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}



#[derive(Shrinkwrap, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringUtf16(pub String);

impl StringUtf16 {
    #[inline]
    pub fn len_tagged(&self) -> Tagged<i16> {
        Tagged::new(self.encode_utf16().count() as i16)
    }

    #[inline]
    pub fn bytes_encoded(&self) -> Vec<u8> {
        self.0.encode_utf16()
            .map(|b| b.to_be_bytes())
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl Encode for StringUtf16 {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        self.len_tagged().encode_into(stream)?;
        stream.write_all(&self.bytes_encoded())?;
        Ok(())
    }
}

impl DynSizedEncode for StringUtf16 {
    #[inline]
    fn encoded_size(&self) -> usize {
        let len = self.len_tagged();
        len.encoded_size() + len.into_inner() as usize * 2
    }
}

impl EncodingTag for StringUtf16 {
    const ENCODING_TAG: [u8; 2] = [0x9C, 0x16];
}

impl Decode for StringUtf16 {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let len_tagged = Tagged::<i16>::decode_from(stream).context("Failed to decode UTF16 string length")?;
        let len = len_tagged.into_inner() as usize;

        let mut content_buf = vec![0u8; len * 2];
        stream.read_exact(&mut content_buf).context("Failed to read UTF16 string contents")?;

        if content_buf.len() % 2 != 0 {
            bail!("Uneven byte length content retrieved for UTF16 string");
        }

        let decoded_content = content_buf.chunks_exact(2)
            .map(|hilo| <[u8; 2]>::try_from(hilo).unwrap())
            .map(|hilo| u16::from_be_bytes(hilo))
            .collect::<Vec<_>>();

        let s = String::from_utf16_lossy(decoded_content.as_slice());
        Ok(Self(s))
    }
}

impl std::fmt::Display for StringUtf16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for StringUtf16 {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for StringUtf16 {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}



#[derive(Shrinkwrap, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnknownTag(pub [u8; 2]);

impl From<[u8; 2]> for UnknownTag {
    fn from(value: [u8; 2]) -> Self {
        Self(value)
    }
}

impl Encode for UnknownTag {
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&self.0)?;
        Ok(())
    }
}

impl ConstSizedEncode for UnknownTag {
    const ENCODED_SIZE: usize = 2;
}

// not implementing Decode as this is a de facto error type



#[derive(Shrinkwrap, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tagged<T> {
    inner: T
}

impl<T> Tagged<T> {
    #[inline]
    pub fn new(tagged: T) -> Self {
        Self {
            inner: tagged
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }
} 

impl<T> From<T> for Tagged<T>
where T: EncodingTag {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Encode for Tagged<T>
where T: Encode + EncodingTag {
    #[inline]
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&T::ENCODING_TAG)?;
        self.inner.encode_into(stream)
    }
}

impl<T> DynSizedEncode for Tagged<T>
where T: DynSizedEncode + EncodingTag {
    #[inline]
    fn encoded_size(&self) -> usize {
        T::ENCODING_TAG.len() + self.inner.encoded_size()
    }
}

impl<T> Decode for Tagged<T> 
where T: Decode + EncodingTag {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).context("Failed to read encoding tag")?;
        if buf != T::ENCODING_TAG {
            bail!("Invalid encoding tag");
        }

        let inner = T::decode_from(stream)?;
        Ok(Self::new(inner))
    }
}

impl<T> std::fmt::Display for Tagged<T>
where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}
