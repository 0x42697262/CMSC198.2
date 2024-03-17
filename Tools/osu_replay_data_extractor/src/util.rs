pub fn collect_files(
    input_directory: &std::path::PathBuf,
) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    if !input_directory.exists() {
        return Ok(vec![]);
    }

    if !input_directory.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Input path is not a directory.",
        ));
    }

    let files = std::fs::read_dir(input_directory)?
        .filter_map(|file| file.ok())
        .map(|file| file.path())
        .filter(|file| file.is_file())
        .collect();

    Ok(files)
}

pub fn read_bool(file: &mut std::fs::File) -> std::io::Result<bool> {
    let mut buffer: [u8; 1] = [0u8; 1];
    std::io::Read::read_exact(file, &mut buffer)?;

    match buffer[0] {
        0 => Ok(false),
        _ => Ok(true),
    }
}

pub fn read_byte(file: &mut std::fs::File) -> std::io::Result<u8> {
    let mut buffer: [u8; 1] = [0u8; 1];
    std::io::Read::read_exact(file, &mut buffer)?;

    Ok(buffer[0])
}

pub fn read_byte_array(file: &mut std::fs::File, size: usize) -> std::io::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0u8; size];
    std::io::Read::read_exact(file, &mut buffer)?;

    Ok(buffer)
}

pub fn read_integer(file: &mut std::fs::File) -> std::io::Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [0u8; 4];
    std::io::Read::read_exact(file, &mut buffer)?;

    Ok(buffer)
}

pub fn read_short(file: &mut std::fs::File) -> std::io::Result<[u8; 2]> {
    let mut buffer: [u8; 2] = [0u8; 2];
    std::io::Read::read_exact(file, &mut buffer)?;

    Ok(buffer)
}

pub fn read_long(file: &mut std::fs::File) -> std::io::Result<[u8; 8]> {
    let mut buffer: [u8; 8] = [0u8; 8];
    std::io::Read::read_exact(file, &mut buffer)?;

    Ok(buffer)
}

pub fn read_string(file: &mut std::fs::File) -> std::io::Result<String> {
    let mut flag: [u8; 1] = [0u8; 1];
    std::io::Read::read_exact(file, &mut flag)?;
    if flag[0] == 0x00 {
        return Ok(String::new());
    }

    if flag[0] != 0x0b {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Not a valid string.",
        ));
    }
    // let mut size: [u8; 1] = [0u8; 1];
    // std::io::Read::read_exact(file, &mut size)?;
    let size: u64 = parse_uleb128(file).unwrap();

    if size == 0 {
        return Ok(String::new());
    }
    // println!("{size:?}");

    let mut buffer: Vec<u8> = vec![0u8; size as usize];
    std::io::Read::read_exact(file, &mut buffer)?;
    let string = String::from_utf8(buffer);
    match string {
        Ok(hash) => Ok(hash),
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not a valid string.",
            ));
        }
    }
}

pub fn parse_uleb128(file: &mut std::fs::File) -> std::io::Result<u64> {
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    let mut buffer: Vec<u8> = vec![0u8; 1];
    loop {
        std::io::Read::read_exact(file, &mut buffer)?;
        let byte: u64 = buffer[0] as u64;
        result = result | ((byte & 0b01111111) << shift) as u64;
        if (byte & 0b10000000) == 0x0 {
            break;
        }
        shift += 7;
    }
    Ok(result)
}

pub fn decompress_lzma(compressed_data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use std::io::Read;

    let mut decoder = xz2::read::XzDecoder::new(compressed_data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(decompressed_data)
}
