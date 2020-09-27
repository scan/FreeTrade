use std::io::{Seek, Write, Cursor, Read};
use std::iter::Iterator;
use std::path::Path;
use std::fs::File;

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = walkdir::DirEntry>,
    prefix: &str,
    writer: T,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    use zip::write::FileOptions;

    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Bzip2).unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path().strip_prefix(Path::new(prefix)).unwrap();
        let name = path.to_str().unwrap();

        if path.is_file() {
            log::info!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if path.as_os_str().len() != 0 {
            log::info!("adding directory {:?} as {:?} ...", path, name);
            zip.add_directory(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn zip_resources() -> zip::result::ZipResult<Vec<u8>> {
    let mut writer = Cursor::new(Vec::new());

    let src_dir = "./resources";
    if !Path::new(src_dir).is_dir() {
        return Result::Err(zip::result::ZipError::FileNotFound);
    }

    let wdir = walkdir::WalkDir::new(src_dir.to_string());
    let it = wdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, writer)?;

    writer.flush()?;

    let mut bytes = Vec::new();
    writer.read_to_end(&mut bytes)?;

    Result::Ok(bytes)
}