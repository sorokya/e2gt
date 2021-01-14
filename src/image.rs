use super::IMAGE_SIZE;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result};

#[derive(Debug)]
pub struct Image {
    pub hash: [u8; 8],
    pub id: u16,
    pub x_offset: u16,
    pub width: u16,
    pub height: u16,
}

impl Image {
    pub fn get_file_name(&self) -> String {
        let mut file_name = String::with_capacity(20);
        for byte in self.hash.iter() {
            file_name.push_str(&(format!("{:#04x}", byte)[2..]));
        }
        file_name.push_str(".png");
        file_name
    }
    pub fn deserialize<R: Read>(buf: &mut R) -> Result<Self> {
        let mut hash: [u8; 8] = [0; 8];
        buf.read_exact(&mut hash)?;

        let id = buf.read_u16::<BigEndian>()?;
        let x_offset = buf.read_u16::<BigEndian>()?;
        let width = buf.read_u16::<BigEndian>()?;
        let height = buf.read_u16::<BigEndian>()?;
        Ok(Self {
            hash,
            id,
            x_offset,
            width,
            height,
        })
    }
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(IMAGE_SIZE);
        buf.extend(self.hash.iter());
        buf.write_u16::<BigEndian>(self.id)?;
        buf.write_u16::<BigEndian>(self.x_offset)?;
        buf.write_u16::<BigEndian>(self.width)?;
        buf.write_u16::<BigEndian>(self.height)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn deserialize() -> Result<()> {
        let mut buf = get_image_buf()?;
        let deserialized = Image::deserialize(&mut buf)?;
        assert_eq!(deserialized.id, 2);
        assert_eq!(
            deserialized.hash,
            [0xD4, 0x05, 0x9D, 0xA8, 0xA8, 0xFB, 0x54, 0x63]
        );
        assert_eq!(deserialized.x_offset, 5);
        assert_eq!(deserialized.width, 100);
        assert_eq!(deserialized.height, 200);

        Ok(())
    }

    #[test]
    fn file_name() {
        let image = get_image();
        assert_eq!(image.get_file_name(), "d4059da8a8fb5463.png");
    }

    fn get_image() -> Image {
        Image {
            id: 2,
            hash: [0xD4, 0x05, 0x9D, 0xA8, 0xA8, 0xFB, 0x54, 0x63],
            x_offset: 5,
            width: 100,
            height: 200,
        }
    }

    fn get_image_buf() -> Result<Cursor<Vec<u8>>> {
        let image = get_image();
        Ok(Cursor::new(image.serialize()?))
    }
}
