#!/usr/bin/env python
#
# Simplified version of the mktables.py script at https://wiki.nesdev.com/w/index.php/APU_period_table
#
# Lookup table generator for note periods
# Copyright 2010 Damian Yerrick
#
# Copying and distribution of this file, with or without
# modification, are permitted in any medium without royalty
# provided the copyright notice and this notice are preserved.
# This file is offered as-is, without any warranty.

lowestFreq = 55.0
ntscOctaveBase = 39375000.0 / (22 * 16 * lowestFreq)
palOctaveBase = 266017125.0 / (10 * 16 * 16 * lowestFreq)
maxNote = 88


def make_period_table(pal=False):
    semitone = 2.0 ** (1. / 12)
    octaveBase = palOctaveBase if pal else ntscOctaveBase
    relFreqs = [(1 << (i // 12)) * semitone ** (i % 12)
                for i in range(maxNote)]
    periods = [int(round(octaveBase / freq)) - 1 for freq in relFreqs]
    for i in range(0, maxNote, 12):
        print(','.join('0x%04x' % i for i in periods[i:i + 12]))


if __name__ == '__main__':
    make_period_table()
