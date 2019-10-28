if __name__ == '__main__':
    twelfth_root_of_two = 2 ** (1.0/12.0)

    base_freq = 27.5  # A0

    for i in range(88):
        if i % 12 == 0:
            freq = 27.5 * 2 ** (i/12)
        else:
            freq = base_freq

        print("%s," % freq)
        base_freq *= twelfth_root_of_two
