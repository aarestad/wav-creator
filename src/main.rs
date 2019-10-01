extern crate byteorder;

use std::{io};
use std::fs::File;
use std::f64::consts::PI;
use byteorder::{BigEndian, WriteBytesExt, LittleEndian};

//const FRAMES_PER_BUFFER: u32 = 512;
//const NUM_CHANNELS: i32 = 1;
const SAMPLE_RATE: i32 = 44_100;
const TAU: f64 = 2.0 * PI;

fn main() -> io::Result<()> {
    let mut wav_output_file = File::create("a_four_forty.wav")?;

    let format_chunk_size = 16;
    let header_size = 8;
    let format_type = 1i16;
    let tracks = 1i16;
    let bits_per_sample = 16i16;
    let frame_size = tracks * ((bits_per_sample + 7) / 8);
    let ms_duration = 1000;

    let bytes_per_second = SAMPLE_RATE * frame_size as i32;
    let wave_size = 4;
    let samples = SAMPLE_RATE * ms_duration / 1000;
    let data_chunk_size = samples * frame_size as i32;
    let chunk_size = wave_size + header_size + format_chunk_size + header_size + data_chunk_size;

    // HEADER
    wav_output_file.write_u32::<BigEndian>(0x46464952)?; // RIFF big-endian ("FFIR")
    wav_output_file.write_i32::<LittleEndian>(chunk_size)?;
    wav_output_file.write_u32::<BigEndian>(0x45564157)?; // WAVE big-endian

    // "fmt " subchunk
    wav_output_file.write_u32::<BigEndian>(0x20746D66)?; // "fmt " big-endian
    wav_output_file.write_i32::<LittleEndian>(format_chunk_size);
//    writer.Write(formatType);
//    writer.Write(tracks);
//    writer.Write(samplesPerSecond);
//    writer.Write(bytes_per_second);
//    writer.Write(frameSize);
//    writer.Write(bitsPerSample);

    // "data" subchunk
//    writer.Write(0x61746164); // = encoding.GetBytes("data")
//    writer.Write(data_chunk_size);
//    {
//        double theta = frequency * TAU / (double)samplesPerSecond;
//        // 'volume' is UInt16 with range 0 thru Uint16.MaxValue ( = 65 535)
//        // we need 'amp' to have the range of 0 thru Int16.MaxValue ( = 32 767)
//        double amp = volume >> 2; // so we simply set amp = volume / 2
//        for (int step = 0; step < samples; step++)
//        {
//            short s = (short)(amp * Math.Sin(theta * (double)step));
//            writer.Write(s);
//        }
//    }
//
//    mStrm.Seek(0, SeekOrigin.Begin);
//    new System.Media.SoundPlayer(mStrm).Play();
//    writer.Close();
//    mStrm.Close();

    Ok(())
}


// stolen from https://stackoverflow.com/questions/203890/creating-sine-or-square-wave-in-c-sharp
/* using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Windows.Forms;

public static void PlayBeep(UInt16 frequency, int msDuration, UInt16 volume = 16383)
{
    var mStrm = new MemoryStream();
    BinaryWriter writer = new BinaryWriter(mStrm);

    const double TAU = 2 * Math.PI;
    int formatChunkSize = 16;
    int headerSize = 8;
    short formatType = 1;
    short tracks = 1;
    int samplesPerSecond = 44100;
    short bitsPerSample = 16;
    short frameSize = (short)(tracks * ((bitsPerSample + 7) / 8));
    int bytesPerSecond = samplesPerSecond * frameSize;
    int waveSize = 4;
    int samples = (int)((decimal)samplesPerSecond * msDuration / 1000);
    int dataChunkSize = samples * frameSize;
    int fileSize = waveSize + headerSize + formatChunkSize + headerSize + dataChunkSize;
    // var encoding = new System.Text.UTF8Encoding();
    writer.Write(0x46464952); // = encoding.GetBytes("RIFF")
    writer.Write(fileSize);
    writer.Write(0x45564157); // = encoding.GetBytes("WAVE")
    writer.Write(0x20746D66); // = encoding.GetBytes("fmt ")
    writer.Write(formatChunkSize);
    writer.Write(formatType);
    writer.Write(tracks);
    writer.Write(samplesPerSecond);
    writer.Write(bytesPerSecond);
    writer.Write(frameSize);
    writer.Write(bitsPerSample);
    writer.Write(0x61746164); // = encoding.GetBytes("data")
    writer.Write(dataChunkSize);
    {
        double theta = frequency * TAU / (double)samplesPerSecond;
        // 'volume' is UInt16 with range 0 thru Uint16.MaxValue ( = 65 535)
        // we need 'amp' to have the range of 0 thru Int16.MaxValue ( = 32 767)
        double amp = volume >> 2; // so we simply set amp = volume / 2
        for (int step = 0; step < samples; step++)
        {
            short s = (short)(amp * Math.Sin(theta * (double)step));
            writer.Write(s);
        }
    }

    mStrm.Seek(0, SeekOrigin.Begin);
    new System.Media.SoundPlayer(mStrm).Play();
    writer.Close();
    mStrm.Close();
}

*/
