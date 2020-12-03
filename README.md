# uff-parser

A UFF58 file parser intended for use with vibration data.
Extracts key fields (frequency spacing, number of measurements and amplitude data and misc. data)

Hopefully this package will:
* Have multithreading support (for decompressing and reading from multiple targz archives)
* Outperform existing UFF readers (pyuff for python as well as my own python code)

Currently working on:
* Base functionality
