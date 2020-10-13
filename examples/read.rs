use exiftool::exifdata::ExifData;
use exiftool::ExifTool;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tool = ExifTool::new("examples/kaiping-diaolou.jpg");
    let res: ExifData = tool.exec()?;
    for (k, v) in res.iter() {
        println!("{:30}: {}", k, v);
    }

    Ok(())
}
