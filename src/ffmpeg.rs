use std::process::Command as ProcessCommand;

/// Processes a video file by adjusting its volume, sample rate, channels, and bitrate using ffmpeg.
///
/// # Arguments
///
/// * `input_path` - A string slice that holds the path to the input video file.
/// * `output_path` - A string slice that holds the path to the output video file.
/// * `volume_change` - An integer that specifies the volume change in dB (can be positive or negative).
/// * `sample_rate` - An unsigned integer that specifies the audio sample rate in Hz.
/// * `channels` - An unsigned integer that specifies the number of audio channels.
/// * `bitrate` - A string slice that specifies the audio bitrate.
pub fn process_file(
    input_path: &str,
    output_path: &str,
    volume_change: i32,
    sample_rate: u32,
    channels: u32,
    bitrate: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // construct the ffmpeg command
    let command = ProcessCommand::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c")
        .arg("copy") // copy video codec (no re-encoding)
        .arg("-c:a")
        .arg("libmp3lame") // audio codec: MP3
        .arg("-ab")
        .arg(bitrate) // audio bitrate
        .arg("-ac")
        .arg(&channels.to_string()) // audio channels (typically 2, for stereo)
        .arg("-ar")
        .arg(&sample_rate.to_string()) // audio sample rate
        .arg("-af")
        .arg(format!("volume={}dB", volume_change)) // adjust volume based on user input
        .arg("-strict")
        .arg("-2") // allow experimental codecs
        .arg("-y") // overwrite output file without asking
        .arg(output_path)
        .status()?; // execute command

    if command.success() {
        println!(
            "Audio/video has been processed successfully.\nOutput saved to: {}",
            output_path
        );
        Ok(())
    } else {
        Err(format!("An error occurred while processing the audio/video.").into())
    }
}
