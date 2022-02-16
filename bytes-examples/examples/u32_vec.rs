use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    // u32 字节序处理
    //      pub const fn to_be(self) -> u32, 转为大端字节序
    //      pub const fn to_le(self) -> u32, 转为小端字节序
    //      pub const fn to_be_bytes(self) -> [u8; 4], 反转字节序
    //      pub const fn to_le_bytes(self) -> [u8; 4], 反转到小端字节序
    //      pub const fn to_be_bytes(self) -> [u8; 4], 反转到大端字节序

    // u32 转大端 直接用std
    let d: u32 = 300;
    let v = d.to_be_bytes().to_vec();
    print!("v: {:#x?}", &v);

    // u32 转大端 使用 byteorder
    let mut d: Vec<u8> = vec![];
    d.write_u32::<BigEndian>(300).unwrap();
    print!("d: {:#x?}", &d);
}
