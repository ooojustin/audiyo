use clap::{Arg, Command};

fn main() {
    let matches = Command::new("audiyo")
        .version("0.2")
        .author("Justin Garofolo")
        .about("A simple CLI tool to adjust video volume.")
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
            .help("Volume change in dB. Use positive values to increase and negative to decrease (default is +10 dB).")
            .default_value("10")
            .value_parser(clap::value_parser!(i32)))
        .arg(Arg::new("sample-rate")
            .long("sample-rate")
            .help("Audio sample rate in Hz (default is 44.1 kHz).")
            .default_value("44100")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("channels")
            .long("channels")
            .help("Number of audio channels (default is 2 for stereo).")
            .default_value("2")
            .value_parser(clap::value_parser!(u32)))
        .arg(Arg::new("bitrate")
            .long("bitrate")
            .help("Audio bitrate (default is '128k').")
            .default_value("128k"))
        .get_matches();

    // retrieve arguments
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let volume = *matches.get_one::<i32>("volume").unwrap();
    let sample_rate = *matches.get_one::<u32>("sample-rate").unwrap();
    let channels = *matches.get_one::<u32>("channels").unwrap();
    let bitrate = matches.get_one::<String>("bitrate").unwrap();

    // TODO(justin): execute command
}
