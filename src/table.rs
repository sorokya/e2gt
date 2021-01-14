use super::{Image, IMAGE_SIZE, TABLE_SIZE};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Result};
pub struct Table {
    pub magic: [u8; 4],
    pub version: u16,
    pub images: Vec<Image>,
}

impl Table {
    pub fn deserialize<R: Read>(buf: &mut R) -> Result<Self> {
        let mut magic: [u8; 4] = [0; 4];
        buf.read_exact(&mut magic)?;

        let version = buf.read_u16::<BigEndian>()?;
        let number_of_images = buf.read_u16::<BigEndian>()?;
        let mut images: Vec<Image> = Vec::with_capacity(number_of_images.into());
        for _ in 0..number_of_images {
            images.push(Image::deserialize(buf)?);
        }

        Ok(Self {
            magic,
            version,
            images,
        })
    }
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::with_capacity(TABLE_SIZE + IMAGE_SIZE * self.images.len());
        buf.extend(self.magic.iter());
        buf.write_u16::<BigEndian>(self.version)?;
        buf.write_u16::<BigEndian>(self.images.len() as u16)?;
        for image in &self.images {
            buf.append(&mut image.serialize()?);
        }
        println!("{:?}", buf);
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn deserialize() -> Result<()> {
        let mut buf = get_table_buf()?;
        let table = Table::deserialize(&mut buf)?;
        assert_eq!(table.magic, *b"E2GT");
        assert_eq!(table.version, 1);
        assert_eq!(table.images.len(), 1);
        assert_eq!(table.images[0].id, 2);
        assert_eq!(
            table.images[0].hash,
            [0xD4, 0x05, 0x9D, 0xA8, 0xA8, 0xFB, 0x54, 0x63]
        );
        assert_eq!(table.images[0].x_offset, 5);
        assert_eq!(table.images[0].width, 100);
        assert_eq!(table.images[0].height, 200);
        Ok(())
    }

    fn get_table_buf() -> Result<Cursor<Vec<u8>>> {
        let mut table = Table {
            magic: *b"E2GT",
            version: 1,
            images: Vec::with_capacity(1),
        };
        table.images.push(Image {
            id: 2,
            hash: [0xD4, 0x05, 0x9D, 0xA8, 0xA8, 0xFB, 0x54, 0x63],
            x_offset: 5,
            width: 100,
            height: 200,
        });

        Ok(Cursor::new(table.serialize()?))
    }
}
