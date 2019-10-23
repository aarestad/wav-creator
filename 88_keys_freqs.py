if __name__ == '__main__':
    twelfth_root_of_two = 2 ** (1.0/12.0)

    base_freq = 27.5  # A0

    for i in range(88):
        print("%s," % base_freq)
        base_freq *= twelfth_root_of_two
