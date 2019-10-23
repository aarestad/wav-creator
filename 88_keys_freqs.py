if __name__ == '__main__':
    twelfth_root_of_two = 2 ** (1.0/12.0)

    eighty_eight_keys_freqs = [0.0] * 88

    # all the A keys have rational frequencies, based on A4 = 440 Hz
    eighty_eight_keys_freqs[84] = 3520.0
    eighty_eight_keys_freqs[72] = 1760.0
    eighty_eight_keys_freqs[60] = 880.0
    eighty_eight_keys_freqs[48] = 440.0
    eighty_eight_keys_freqs[36] = 220.0
    eighty_eight_keys_freqs[24] = 110.0
    eighty_eight_keys_freqs[12] = 55.0
    eighty_eight_keys_freqs[0] = 27.5

    for i in range(88):
        if eighty_eight_keys_freqs[i] == 0:
            eighty_eight_keys_freqs[i] = eighty_eight_keys_freqs[i-1] * twelfth_root_of_two
        i += 1

    for i, freq in enumerate(eighty_eight_keys_freqs):
        print("%s," % (freq,))
