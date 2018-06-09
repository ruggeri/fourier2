# Done

1. Applies fourier transform to recover musical notes.
2. Works well in face of substantial Gaussian noise.
    * Makes sense, since this noise is symmetric.
3. Can read in sound data.
4. Can generate musical samples.
5. Does some basic smoothing to find most important tones.

# TODO

1. Refactor SmoothedPitchIterator. It's trash.
2. Can't I write HzScanner and ScaleScanner as iterator map operations?
3. Could consider if SongIterator can be written more easily (but probably not?).
4. PCMFile should ideally implement an Fn trait so I can pass it.
