use byteorder::{LittleEndian, WriteBytesExt};
use sample::{signal, Signal};
use std::fs::File;
use std::i16;
use std::io;
use std::io::Write;
use std::path::Path;

/// Chunk labels
const RIFF_LABEL: &[u8] = b"RIFF";
const FORMAT_LABEL: &[u8] = b"WAVE";
const DATA_LABEL: &[u8] = b"data";
const FMT_LABEL: &[u8] = b"fmt ";

/// metadata chunk is always 16 bytes
const FMT_CHUNK_SIZE: u32 = 16u32;
/// 8 bytes for 4-byte string label plus 4-byte subchunk size
const HEADER_SIZE: u32 = 8u32;
/// 1 means PCM
const FORMAT_TYPE: u16 = 1u16;
/// Standard sample rate: 44.1 KHz
const SAMPLE_RATE: u32 = 44_100u32;
/// Standard 16-bit sound resolution
const BITS_PER_SAMPLE: u16 = 16u16;

fn main() -> io::Result<()> {
    write(
        1,
        1000,
        440.0,
        i16::MAX.into(),
        Path::new("a_four_forty.wav"),
    )
}

fn write(
    num_channels: u16,
    ms_duration: u32,
    freq: f64,
    amp: f64,
    file_name: &Path,
) -> io::Result<()> {
    let bytes_per_sample: u16 = (num_channels * BITS_PER_SAMPLE) / 8;
    let num_samples: u32 = SAMPLE_RATE * ms_duration / 1000;
    let data_chunk_size: u32 = num_samples * (bytes_per_sample as u32);
    let file_size: u32 = 4 + HEADER_SIZE + FMT_CHUNK_SIZE + HEADER_SIZE + data_chunk_size;

    let mut wav_output_file = File::create(file_name)?;

    // HEADER
    // 0         4   ChunkID          Contains the letters "RIFF" in ASCII form (0x52494646 big-endian form).
    // 4         4   ChunkSize        36 + SubChunk2Size, or more precisely: 4 + (8 + SubChunk1Size) + (8 + SubChunk2Size) This is the size of the rest of the chunk following this number.  This is the size of the entire file in bytes minus 8 bytes for the two fields not included in this count: ChunkID and ChunkSize.
    // 8         4   Format           Contains the letters "WAVE" (0x57415645 big-endian form).
    wav_output_file.write(RIFF_LABEL)?;
    wav_output_file.write_u32::<LittleEndian>(file_size)?;
    wav_output_file.write(FORMAT_LABEL)?; // WAVE big-endian

    // "fmt " subchunk
    // 12        4   Subchunk1ID      Contains the letters "fmt " (0x666d7420 big-endian form).
    // 16        4   Subchunk1Size    16 for PCM.  This is the size of the rest of the Subchunk which follows this number.
    // 20        2   AudioFormat      PCM = 1 (i.e. Linear quantization) Values other than 1 indicate some form of compression.
    // 22        2   NumChannels      Mono = 1, Stereo = 2, etc.
    // 24        4   SampleRate       8000, 44100, etc.
    // 28        4   ByteRate         == SampleRate * NumChannels * BitsPerSample/8
    // 32        2   BlockAlign       == NumChannels * BitsPerSample/8 The number of bytes for one sample including all channels. I wonder what happens when this number isn't an integer?
    // 34        2   BitsPerSample    8 bits = 8, 16 bits = 16, etc.
    wav_output_file.write(FMT_LABEL)?;
    wav_output_file.write_u32::<LittleEndian>(FMT_CHUNK_SIZE)?;
    wav_output_file.write_u16::<LittleEndian>(FORMAT_TYPE)?;
    wav_output_file.write_u16::<LittleEndian>(num_channels)?;
    wav_output_file.write_u32::<LittleEndian>(SAMPLE_RATE)?;
    wav_output_file.write_u32::<LittleEndian>(SAMPLE_RATE * (bytes_per_sample as u32))?;
    wav_output_file.write_u16::<LittleEndian>(bytes_per_sample)?;
    wav_output_file.write_u16::<LittleEndian>(BITS_PER_SAMPLE)?;

    // "data" subchunk
    // 36        4   Subchunk2ID      Contains the letters "data" (0x64617461 big-endian form).
    // 40        4   Subchunk2Size    == NumSamples * NumChannels * BitsPerSample/8 This is the number of bytes in the data. You can also think of this as the size of the read of the subchunk following this number.
    // 44        *   Data             The actual sound data.
    wav_output_file.write(DATA_LABEL)?;
    wav_output_file.write_u32::<LittleEndian>(data_chunk_size)?;

    let mut signal = signal::rate(SAMPLE_RATE.into())
        .const_hz(freq)
        .sine()
        .scale_amp(amp);

    for _ in 0..num_samples {
        let sample = signal.next()[0];

        wav_output_file.write_i16::<LittleEndian>(sample as i16)?;
    }

    Ok(())
}
