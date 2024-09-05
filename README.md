# audiyo

Open source command line utility to adjust volume, sample rate, channels, and bitrate of audio/video files.

Written in Rust, and uses [FFmpeg](https://www.ffmpeg.org/) behind the scenes.

## Usage

You can get started by using the `audiyo` command to adjust the volume of an audio or video file.

For example, to decrease the volume by 10 dB, use:

```bash
audiyo input.mp4 output.mp4 --volume "-10"
```

Or to increase the volume by 5 dB:

```bash
audiyo input.mp4 output.mp4 --volume "5"
```

## Arguments

| Command              | Description                                                       | Default |
| -------------------- | ----------------------------------------------------------------- | ------- |
| --volume <VALUE>     | Change volume in dB (positive to increase, negative to decrease). | 10      |
| --sample-rate <RATE> | Set the audio sample rate in Hz (e.g., 44100 for 44.1 kHz).       | 44100   |
| --channels <NUMBER>  | Set the number of audio channels (e.g., 2 for stereo).            | 2       |
| --bitrate <BITRATE>  | Set the audio bitrate (e.g., 128k).                               | 128k    |
| -h, --help           | Print help information.                                           | 👋      |

For more details, the `audiyo --help` command may be useful.
It will output the following information:

```
$ audiyo --help
Open source command line utility to adjust volume, sample rate, channels, and bitrate of audio/video files.

Usage: audiyo.exe [OPTIONS] <input> <output>

Arguments:
  <input>   Path to the input video file.
  <output>  Path to the output video file.

Options:
  -v, --volume <volume>            Volume change in dB. Use positive values to increase and negative to decrease. [default: 10]
      --sample-rate <sample-rate>  Audio sample rate in Hz. [default: 44100]
      --channels <channels>        Number of audio channels. [default: 2]
      --bitrate <bitrate>          Audio bitrate. [default: 128k]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Questions/Suggestions

If you have any questions about this program or want to provide feedback, feel free to:

- ❓ [Create an issue](https://github.com/ooojustin/audiyo/issues) on GitHub.
- 📫 Reach me via email: [justin@garofolo.net](mailto:justin@garofolo.net)
