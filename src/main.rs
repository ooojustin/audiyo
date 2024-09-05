use clap::{Arg, Command};
use std::path::Path;

mod audio;
use audio::process_file;

fn main() {
    let matches = Command::new("audiyo")
        .version("0.2")
        .author("Justin Garofolo")
        .about("Open source command line utility to adjust volume, sample rate, channels, and bitrate of audio/video files.")
        .arg(Arg::new("input")
            .help("Path to the input video file.")
            .required(true)
            .index(1))
        .arg(Arg::new("output")
            .help("Path to the output video file.")
            .required(true)
            .index(2))
        .arg(Arg::new("volume")
            .long("volume")
            .short('v')
            .help("Volume change in dB. Use positive values to increase and negative to decrease.")
            .default_value("10")
            .value_parser(clap::value_parser!(i32)))
        .arg(Arg::new("sample-rate")
            .long("sample-rate")
            .help("Audio sample rate in Hz.")
            .default_value("44100")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("channels")
            .long("channels")
            .help("Number of audio channels.")
            .default_value("2")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("bitrate")
            .long("bitrate")
            .help("Audio bitrate.")
            .default_value("128k"))
        .get_matches();

    // retrieve arguments
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let volume = *matches.get_one::<i32>("volume").unwrap();
    let sample_rate = *matches.get_one::<u32>("sample-rate").unwrap();
    let channels = *matches.get_one::<u32>("channels").unwrap();
    let bitrate = matches.get_one::<String>("bitrate").unwrap();

    // ensure input file exists
    if !Path::new(input).exists() {
        eprintln!("Error: Input file '{}' not found.", input);
        return;
    }

    // process the file using the specified arguments
    if let Err(err) = process_file(input, output, volume, sample_rate, channels, bitrate) {
        eprintln!("Error: {}", err);
    }
}
