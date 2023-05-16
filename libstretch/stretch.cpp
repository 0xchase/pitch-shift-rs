#include "stretch.h"
#include <stdlib.h>
#include <stdio.h>

typedef signalsmith::stretch::SignalsmithStretch<float> PitchShifter;

extern "C" PitchShifter* pitch_shifter_create() {
    return new PitchShifter();
}

extern "C" void pitch_shifter_destroy(PitchShifter* shifter) {
    delete shifter;
}

extern "C" void pitch_shifter_set_transpose_factor(PitchShifter* shifter, float factor) {
    shifter->setTransposeFactor(factor);
}

extern "C" void pitch_shifter_set_transpose_semitones(PitchShifter* shifter, float semitones) {
    shifter->setTransposeSemitones(semitones);
}

extern "C" void pitch_shifter_process(PitchShifter* shifter, float** input, int inputSamples, float** output, int outputSamples) {
    shifter->process(input, inputSamples, output, outputSamples);
}

extern "C" void pitch_shifter_prepare_default(PitchShifter* shifter, int channels, float sampleRate) {
    shifter->presetDefault(channels, sampleRate);
}

extern "C" void pitch_shifter_prepare_cheaper(PitchShifter* shifter, int channels, float sampleRate) {
    shifter->presetCheaper(channels, sampleRate);
}

extern "C" void pitch_shifter_prepare_custom(PitchShifter* shifter, int channels, int blockSamples, int intervalSamples) {
    shifter->configure(channels, blockSamples, intervalSamples);
}

extern "C" void pitch_shifter_reset(PitchShifter* shifter) {
    shifter->reset();
}

int main() {
    return 0;
}
