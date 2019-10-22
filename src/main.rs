use byteorder::{LittleEndian, WriteBytesExt};
use sample::{signal, Signal};
use std::fs::File;
use std::io;
use std::io::Write;
use std::i16;

const CHUNK_LABEL: &[u8] = "RIFF".as_bytes();
const FORMAT_LABEL: &[u8] = "WAVE".as_bytes();
const DATA_LABEL: &[u8] = "data".as_bytes();
const FMT_LABEL: &[u8] = "fmt ".as_bytes();

const PCM_CHUNK_SIZE: u32 = 16u32; // for PCM

const HEADER_SIZE: u32 = 8u32; // 4 bytes for id + 4 bytes for size

fn main() -> io::Result<()> {

    let format_type = 1u16; // Linear PCM
    let num_channels = 1u16; // Mono; 2 is stereo
    let sample_rate = 44_100u32;
    let bits_per_sample = 16u16;
    let block_align = (num_channels * bits_per_sample) / 8;
    let byte_rate = sample_rate * num_channels as u32 * bits_per_sample as u32 / 8;
    let ms_duration = 1000u32;

    let wave_size = 4;
    let samples = sample_rate * ms_duration / 1000;
    let data_chunk_size = samples * block_align as u32;
    let file_size = wave_size + HEADER_SIZE + PCM_CHUNK_SIZE + HEADER_SIZE + data_chunk_size;

    let mut wav_output_file = File::create("a_four_forty.wav")?;

    // HEADER
    // 0         4   ChunkID          Contains the letters "RIFF" in ASCII form (0x52494646 big-endian form).
    // 4         4   ChunkSize        36 + SubChunk2Size, or more precisely: 4 + (8 + SubChunk1Size) + (8 + SubChunk2Size) This is the size of the rest of the chunk following this number.  This is the size of the entire file in bytes minus 8 bytes for the two fields not included in this count: ChunkID and ChunkSize.
    // 8         4   Format           Contains the letters "WAVE" (0x57415645 big-endian form).
    wav_output_file.write(CHUNK_LABEL)?;
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
    wav_output_file.write_u32::<LittleEndian>(PCM_CHUNK_SIZE)?;
    wav_output_file.write_u16::<LittleEndian>(format_type)?;
    wav_output_file.write_u16::<LittleEndian>(num_channels)?;
    wav_output_file.write_u32::<LittleEndian>(sample_rate)?;
    wav_output_file.write_u32::<LittleEndian>(byte_rate)?;
    wav_output_file.write_u16::<LittleEndian>(block_align)?;
    wav_output_file.write_u16::<LittleEndian>(bits_per_sample)?;

    // "data" subchunk
    // 36        4   Subchunk2ID      Contains the letters "data" (0x64617461 big-endian form).
    // 40        4   Subchunk2Size    == NumSamples * NumChannels * BitsPerSample/8 This is the number of bytes in the data. You can also think of this as the size of the read of the subchunk following this number.
    // 44        *   Data             The actual sound data.
    wav_output_file.write(DATA_LABEL)?;
    wav_output_file.write_u32::<LittleEndian>(data_chunk_size)?;

    let mut signal = signal::rate(sample_rate as f64)
        .const_hz(440.0)
        .sine()
        .scale_amp(i16::MAX as f64);

    let mut signal_counter = 0;

    while signal_counter < sample_rate {
        let sample = signal.next()[0] as i16;

        println!("{:?}", sample);
        wav_output_file.write_i16::<LittleEndian>(sample)?;
        signal_counter += 1;
    }

    Ok(())
}
