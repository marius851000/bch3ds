//! Various helper function to read the BCH/H3D file format
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub fn read_null_terminated_ascii_string<T: Read>(file: &mut T) -> Result<String, io::Error> {
    let mut result = String::new();
    let mut buffer = [0];
    loop {
        file.read_exact(&mut buffer)?;
        if buffer[0] == 0 {
            if cfg!(feature = "compare") {
                println!("found string {}", result);
            };
            return Ok(result);
        };
        result.push(buffer[0] as char)
    }
}

pub fn read_u8<T: Read>(file: &mut T) -> Result<u8, io::Error> {
    let mut buffer = [0];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found u8 {}", buffer[0]);
    };
    Ok(buffer[0])
}

pub fn read_i8<T: Read>(file: &mut T) -> Result<i8, io::Error> {
    let mut buffer = [0];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found i8 {}", buffer[0]);
    };
    Ok(buffer[0] as i8)
}

pub fn read_u16_le<T: Read>(file: &mut T) -> Result<u16, io::Error> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found u16 {}", u16::from_le_bytes(buffer));
    };
    Ok(u16::from_le_bytes(buffer))
}

pub fn read_i16_le<T: Read>(file: &mut T) -> Result<i16, io::Error> {
    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found i16 {}", u16::from_le_bytes(buffer));
    };
    Ok(i16::from_le_bytes(buffer))
}

pub fn read_u32_le<T: Read>(file: &mut T) -> Result<u32, io::Error> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found u32 {}", u32::from_le_bytes(buffer));
    };
    Ok(u32::from_le_bytes(buffer))
}

pub fn read_i32_le<T: Read>(file: &mut T) -> Result<i32, io::Error> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found i32 {}", i32::from_le_bytes(buffer));
    };
    Ok(i32::from_le_bytes(buffer))
}

pub fn read_f32_le<T: Read>(file: &mut T) -> Result<f32, io::Error> {
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found Single {}", f32::from_le_bytes(buffer));
    };
    Ok(f32::from_le_bytes(buffer))
}

pub fn read_i8_le<T: Read>(file: &mut T) -> Result<i8, io::Error> {
    let mut buffer = [0];
    file.read_exact(&mut buffer)?;
    if cfg!(feature = "compare") {
        println!("found SByte {}", i8::from_le_bytes(buffer));
    };
    Ok(i8::from_le_bytes(buffer))
}

pub fn read_referenced<F, C, O>(file: &mut F, command: C) -> Result<Option<O>, io::Error>
where
    F: Read + Seek,
    C: Fn(&mut F) -> Result<O, io::Error>,
{
    let mut buf = [0; 4];
    file.read_exact(&mut buf)?;
    let target_offset = u32::from_le_bytes(buf);
    if target_offset == 0 {
        return Ok(None);
    };
    let initial_offset = file.seek(SeekFrom::Current(0))?;

    file.seek(SeekFrom::Start(target_offset as u64))?;
    let result = command(file)?;
    file.seek(SeekFrom::Start(initial_offset))?;
    Ok(Some(result))
}

pub fn read_referenced_null_terminated_ascii_string<T: Read + Seek>(
    file: &mut T,
) -> Result<Option<String>, io::Error> {
    read_referenced(file, |x| read_null_terminated_ascii_string(x))
}

pub fn pad_file<T: Seek>(file: &mut T, divider: u64) -> Result<(), io::Error> {
    let position = file.seek(SeekFrom::Current(0))?;
    let to_skip = (position % divider) as i64;
    file.seek(SeekFrom::Current(to_skip))?;
    Ok(())
}

//TODO: Error
#[derive(Debug)]
pub enum ReadVecError<E: Sized + std::fmt::Debug> {
    ReadingValueError(E),
    IOError(io::Error),
}

impl<E: Sized + std::fmt::Debug> From<io::Error> for ReadVecError<E> {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

pub fn read_vec_distant<F, E, O, P>(file: &mut F, func: P) -> Result<Vec<O>, ReadVecError<E>>
where
    F: Read + Seek,
    E: Sized + std::fmt::Debug,
    P: Fn(&mut F) -> Result<O, E>,
{
    if cfg!(feature = "compare") {
        println!("ignore");
    };
    let nb1 = read_u32_le(file)?;
    if cfg!(feature = "compare") {
        println!("ignore");
    };
    let nb2 = read_u32_le(file)?;
    if cfg!(feature = "compare") {
        println!("ignore");
    };
    let nb3 = read_u32_le(file)?;
    if cfg!(feature = "compare") {
        println!("ignore");
    };
    let nb4 = read_u32_le(file)?;
    println!("1:{}, 2:{}, 3:{}, 4:{}", nb1, nb2, nb3, nb4);
    let position = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(nb1 as u64))?;
    let result = read_vec_inline(file, func, nb2 as u64)?;
    file.seek(SeekFrom::Start(position))?;
    todo!();
}

pub fn read_vec_inline<F, O, E, P>(
    file: &mut F,
    treat: P,
    entries_nb: u64,
) -> Result<Vec<O>, ReadVecError<E>>
where
    F: Read + Seek,
    O: Sized,
    E: Sized + std::fmt::Debug,
    P: Fn(&mut F) -> Result<O, E>,
{
    if cfg!(feature = "compare") {
        println!("this is a list, lenght: {}", entries_nb);
    };
    let mut result_vec: Vec<O> = Vec::new();

    for _ in 0..entries_nb {
        result_vec.push(match treat(file) {
            Ok(value) => value,
            Err(err) => return Err(ReadVecError::ReadingValueError(err)),
        });
    }
    Ok(result_vec)
}

pub fn read_vec_repeat_pointer<T, O, E, P>(
    file: &mut T,
    treat: P,
) -> Result<Vec<O>, ReadVecError<E>>
where
    T: Read + Seek,
    O: Sized + std::fmt::Debug,
    E: Sized + std::fmt::Debug,
    P: Fn(&mut T) -> Result<O, E>,
{
    let mut buf = [0; 4];
    file.read_exact(&mut buf)?;
    file.read_exact(&mut buf)?;
    let entries_nb = u32::from_le_bytes(buf);

    file.read_exact(&mut buf)?;
    let position = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(u32::from_le_bytes(buf) as u64))?;

    let result = read_vec_inline(file, treat, entries_nb as u64)?;

    file.seek(SeekFrom::Start(position))?;

    Ok(result)
}

pub fn read_vec_pointer<T, O, E, P>(
    file: &mut T,
    treat: P,
    entries_nb: u64,
) -> Result<Vec<O>, ReadVecError<E>>
where
    T: Read + Seek,
    O: Sized + std::fmt::Debug,
    E: Sized + std::fmt::Debug,
    P: Fn(&mut T) -> Result<O, E>,
{
    if cfg!(feature = "compare") {
        println!("this is a list, lenght: {}", entries_nb);
    };
    let mut result_vec: Vec<O> = Vec::new();
    let initial_offset = file.seek(SeekFrom::Current(0))?;
    let mut buffer = [0; 4];

    for counter in 0..entries_nb {
        file.seek(SeekFrom::Start(initial_offset + counter * 4))?;
        file.read_exact(&mut buffer)?;
        file.seek(SeekFrom::Start(u32::from_le_bytes(buffer) as u64))?;
        result_vec.push(match treat(file) {
            Ok(value) => value,
            Err(err) => return Err(ReadVecError::ReadingValueError(err)),
        });
    }
    Ok(result_vec)
}

//TODO: Error
#[derive(Debug)]
pub enum ReadBCHDictError<E: Sized + std::fmt::Debug> {
    IOError(io::Error),
    ReadVecError(ReadVecError<E>),
}

impl<E: Sized + std::fmt::Debug> From<io::Error> for ReadBCHDictError<E> {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

#[derive(Debug)]
pub struct BCHDict<T: Sized + std::fmt::Debug> {
    values: Vec<T>,
}

pub fn read_bch_dict<T, O, E, P>(file: &mut T, treat: P) -> Result<BCHDict<O>, ReadBCHDictError<E>>
where
    T: Read + Seek,
    O: Sized + std::fmt::Debug,
    E: Sized + std::fmt::Debug,
    P: Fn(&mut T) -> Result<O, E>,
{
    if cfg!(feature = "compare") {
        println!("calling read on SPICA.Formats.CtrH3D.H3DDict");
    };
    let mut buf = [0; 4];

    file.read_exact(&mut buf)?;
    let vec_offset = i32::from_le_bytes(buf);

    file.read_exact(&mut buf)?;
    let vec_length = i32::from_le_bytes(buf);

    file.read_exact(&mut buf)?;
    let name_offset = i32::from_le_bytes(buf);

    let position = file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(vec_offset as u64))?;

    let returned_vec = match read_vec_pointer(file, treat, vec_length as u64) {
        Ok(out) => out,
        Err(err) => return Err(ReadBCHDictError::ReadVecError(err)),
    };
    println!("{:?}", returned_vec);

    file.seek(SeekFrom::Start(position))?;
    todo!()
    //TODO: name_tree
}

pub fn read_matrix3x4_f32<T: Read>(file: &mut T) -> Result<[[f32; 4]; 3], io::Error> {
    let mut buffer = [0; 4];
    let mut result = [[0.0; 4]; 3];
    for first_counter in 0..3 {
        let mut second_slice = [0.0; 4];
        for second_counter in 0..4 {
            file.read_exact(&mut buffer)?;
            second_slice[second_counter] = f32::from_le_bytes(buffer);
        }
        result[first_counter] = second_slice;
    }
    if cfg!(feature = "compare") {
        println!("found Matrix3x4 {{ {{M11:{} M12:{} M13:{} M14:{}}} {{M21:{} M22:{} M23:{} M24:{}}} {{M31:{} M32:{} M33:{} M34:{}}} {{M41:0 M42:0 M43:0 M44:0}} }}", result[0][0], result[0][1], result[0][2], result[0][3], result[1][0], result[1][1], result[1][2], result[1][3], result[2][0], result[2][1], result[2][2], result[2][3]);
    };
    Ok(result)
}

pub fn read_vector2_f32<T: Read>(file: &mut T) -> Result<[f32; 2], io::Error> {
    let mut buffer = [0; 4];
    let mut result = [0.0; 2];
    for counter in 0..2 {
        file.read_exact(&mut buffer)?;
        result[counter] = f32::from_le_bytes(buffer);
    }
    if cfg!(feature = "compare") {
        println!("found Vector2 <{}  {}>", result[0], result[1]);
    };
    Ok(result)
}

pub fn read_matrix4x3_f32_le<T: Read>(file: &mut T) -> Result<[[f32; 3]; 4], io::Error> {
    let mut result = [[0.0; 3]; 4];
    for column in &mut result {
        for line in 0..3 {
            column[line] = read_f32_le(file)?;
        }
    }
    Ok(result)
}

pub fn read_vector<T, F, R>(file: &mut T, call: F, vector: &mut [R]) -> Result<(), io::Error>
where
    T: Read,
    F: Fn(&mut T) -> Result<R, io::Error>,
{
    for entry in vector.iter_mut() {
        *entry = call(file)?;
    }
    Ok(())
}
