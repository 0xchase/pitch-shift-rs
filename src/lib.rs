pub struct PitchShifter {
    shifter: *mut OpaquePitchShifter,
}

impl PitchShifter {
    pub fn new() -> Self {
        Self {
            shifter: unsafe { pitch_shifter_create() },
        }
    }

    pub fn set_transpose_factor(&mut self, factor: f32) {
        unsafe {
            pitch_shifter_set_transpose_factor(self.shifter, factor);
        }
    }

    pub fn set_transpose_semitones(&mut self, semitones: f32) {
        unsafe {
            pitch_shifter_set_transpose_semitones(self.shifter, semitones);
        }
    }

    pub fn process(&mut self, inputs: *mut *mut f32, input_samples: u32, outputs: *mut *mut f32, output_samples: u32) {
        unsafe {
            pitch_shifter_process(
                self.shifter,
                inputs,
                input_samples,
                outputs,
                output_samples,
            );
        }
    }

    pub fn prepare_default(&mut self, channels: u32, sample_rate: f32) {
        unsafe {
            pitch_shifter_prepare_default(self.shifter, channels, sample_rate);
        }
    }

    pub fn prepare_cheaper(&mut self, channels: u32, sample_rate: f32) {
        unsafe {
            pitch_shifter_prepare_cheaper(self.shifter, channels, sample_rate);
        }
    }

    pub fn prepare_custom(&mut self, channels: u32, block_samples: u32, interval_samples: u32) {
        unsafe {
            pitch_shifter_prepare_custom(self.shifter, channels, block_samples, interval_samples);
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            pitch_shifter_reset(self.shifter);
        }
    }
}

impl Drop for PitchShifter {
    fn drop(&mut self) {
        unsafe {
            pitch_shifter_destroy(self.shifter);
        }
    }
}

#[repr(C)]
struct OpaquePitchShifter(*mut u8);

#[link(name="stretch", kind="static")]
extern "C" {
    fn pitch_shifter_create() -> *mut OpaquePitchShifter;
    fn pitch_shifter_destroy(pitch_shifter: *mut OpaquePitchShifter);
    fn pitch_shifter_set_transpose_factor(shifter: *mut OpaquePitchShifter, factor: f32);
    fn pitch_shifter_set_transpose_semitones(shifter: *mut OpaquePitchShifter, semitones: f32);
    fn pitch_shifter_process(shifter: *mut OpaquePitchShifter, input: *mut *mut f32, inputSamples: u32, output: *mut *mut f32, outputSamples: u32);
    fn pitch_shifter_prepare_default(shifter: *mut OpaquePitchShifter, channels: u32, sampleRate: f32);
    fn pitch_shifter_prepare_cheaper(shifter: *mut OpaquePitchShifter, channels: u32, sampleRate: f32);
    fn pitch_shifter_prepare_custom(shifter: *mut OpaquePitchShifter, channels: u32, blockSamples: u32, intervalSamples: u32);
    fn pitch_shifter_reset(shifter: *mut OpaquePitchShifter);
}
