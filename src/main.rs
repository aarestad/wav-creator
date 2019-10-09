extern crate byteorder;

use std::io;
use std::fs::File;
use std::f64::consts::PI;
use byteorder::{BigEndian, WriteBytesExt, LittleEndian};
use std::io::Write;

//const FRAMES_PER_BUFFER: u32 = 512;
//const NUM_CHANNELS: i32 = 1;
//const SAMPLE_RATE: u32 = 44_100;
//const TAU: f64 = 2.0 * PI;

fn main() -> io::Result<()> {
    let data_str = "data".as_bytes();

    // For each subchunk:
    let header_size = 8u32; // 4 bytes for id + 4 bytes for size

    // For the RIFF main header:
    let chunk_id = "RIFF".as_bytes();
    // Define ChunkSize at the bottom...
    let format_id = "WAVE".as_bytes();

    // For format subchunk:
    let fmt_str  = "fmt ".as_bytes();
    let format_chunk_size = 16u32; // for PCM
    let format_type = 1u16; // Linear PCM
    let num_channels = 1u16; // Mono; 2 is stereo
    let sample_rate = 44_100u32;
    let bits_per_sample = 16u16;
    let byte_rate = sample_rate * num_channels as u32 * bits_per_sample as u32/8;
    let frame_size = num_channels * ((bits_per_sample + 7) / 8);
    let ms_duration = 1000u32;

    let bytes_per_second = sample_rate * frame_size as u32;
    let wave_size = 4;
    let samples = sample_rate * ms_duration / 1000;
    let data_chunk_size = samples * frame_size as u32;
    let file_size = wave_size + header_size + format_chunk_size + header_size + data_chunk_size;

    let mut wav_output_file = File::create("a_four_forty.wav")?;

        // HEADER
        // 0         4   ChunkID          Contains the letters "RIFF" in ASCII form (0x52494646 big-endian form).
        // 4         4   ChunkSize        36 + SubChunk2Size, or more precisely: 4 + (8 + SubChunk1Size) + (8 + SubChunk2Size) This is the size of the rest of the chunk following this number.  This is the size of the entire file in bytes minus 8 bytes for the two fields not included in this count: ChunkID and ChunkSize.
        // 8         4   Format           Contains the letters "WAVE" (0x57415645 big-endian form).
        wav_output_file.write(chunk_id);
        wav_output_file.write_u32::<LittleEndian>(file_size);
        wav_output_file.write(format_id); // WAVE big-endian

        // "fmt " subchunk
        // 12        4   Subchunk1ID      Contains the letters "fmt " (0x666d7420 big-endian form).
        // 16        4   Subchunk1Size    16 for PCM.  This is the size of the rest of the Subchunk which follows this number.
        // 20        2   AudioFormat      PCM = 1 (i.e. Linear quantization) Values other than 1 indicate some form of compression.
        // 22        2   NumChannels      Mono = 1, Stereo = 2, etc.
        // 24        4   SampleRate       8000, 44100, etc.
        // 28        4   ByteRate         == SampleRate * NumChannels * BitsPerSample/8
        // 32        2   BlockAlign       == NumChannels * BitsPerSample/8 The number of bytes for one sample including all channels. I wonder what happens when this number isn't an integer?
        // 34        2   BitsPerSample    8 bits = 8, 16 bits = 16, etc.
        wav_output_file.write(fmt_str);
        wav_output_file.write_u32::<LittleEndian>(format_chunk_size);
        wav_output_file.write_u16::<LittleEndian>(format_type);
        wav_output_file.write_u16::<LittleEndian>(num_channels);
//    writer.Write(samplesPerSecond);
//    writer.Write(bytes_per_second);
//    writer.Write(frameSize);
//    writer.Write(bitsPerSample);

        // "data" subchunk
        // 36        4   Subchunk2ID      Contains the letters "data" (0x64617461 big-endian form).
        // 40        4   Subchunk2Size    == NumSamples * NumChannels * BitsPerSample/8 This is the number of bytes in the data. You can also think of this as the size of the read of the subchunk following this number.
        // 44        *   Data             The actual sound data.
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
