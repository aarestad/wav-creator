import wave

if __name__ == '__main__':
    with wave.open('a_four_forty.wav') as wav_file:
        print('expected 1 channel, got {}'.format(wav_file.getnchannels()))
        print('expected sample width of 2, got {}'.format(wav_file.getsampwidth()))
        print('expected frame rate of 44100, got {}'.format(wav_file.getframerate()))
        print('expected 44100 frames, got {}'.format(wav_file.getnframes()))
        print('params: {}'.format(wav_file.getparams()))
