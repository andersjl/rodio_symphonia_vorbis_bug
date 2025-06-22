use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("test.ogg")?;
    sink.append(rodio::Decoder::try_from(file)?);

    sink.sleep_until_end();

    Ok(())
    /*
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&stream_handle)?;
    let file = std::fs::File::open("test.ogg")?;
    let source = rodio::Decoder::new(std::io::BufReader::new(file))?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
    */
}
