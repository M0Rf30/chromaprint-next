pub mod window;

use realfft::RealFftPlanner;
use realfft::RealToComplex;
use std::sync::Arc;

/// FFT processor that handles frame slicing, windowing, and FFT computation.
pub struct FftProcessor {
    frame_size: usize,
    increment: usize,
    window: Vec<f32>,
    // Ring buffer for incoming samples
    ring_buffer: Vec<i16>,
    ring_write: usize,
    ring_read: usize,
    ring_count: usize,
    // FFT state
    fft: Arc<dyn RealToComplex<f32>>,
    fft_input: Vec<f32>,
    fft_scratch: Vec<realfft::num_complex::Complex<f32>>,
    fft_output: Vec<realfft::num_complex::Complex<f32>>,
    // Power spectrum output
    power_spectrum: Vec<f32>,
}

impl FftProcessor {
    pub fn new(frame_size: usize, frame_overlap: usize) -> Self {
        let increment = frame_size - frame_overlap;
        let window = window::hamming(frame_size);

        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(frame_size);

        let fft_input = vec![0.0f32; frame_size];
        let fft_output = fft.make_output_vec();
        let fft_scratch = fft.make_scratch_vec();
        let power_spectrum = vec![0.0f32; frame_size / 2 + 1];

        // Ring buffer large enough to hold at least 2 frames
        let ring_size = frame_size * 4;

        Self {
            frame_size,
            increment,
            window,
            ring_buffer: vec![0i16; ring_size],
            ring_write: 0,
            ring_read: 0,
            ring_count: 0,
            fft,
            fft_input,
            fft_scratch,
            fft_output,
            power_spectrum,
        }
    }

    pub fn reset(&mut self) {
        self.ring_write = 0;
        self.ring_read = 0;
        self.ring_count = 0;
    }

    /// Feed samples into the FFT processor.
    /// Calls `callback` for each complete FFT frame's power spectrum.
    pub fn consume<F>(&mut self, samples: &[i16], mut callback: F)
    where
        F: FnMut(&[f32]),
    {
        let ring_len = self.ring_buffer.len();

        // Interleave writing with processing to avoid ring buffer overflow.
        for &sample in samples {
            self.ring_buffer[self.ring_write] = sample;
            self.ring_write = (self.ring_write + 1) % ring_len;
            self.ring_count += 1;

            // Process frames as soon as we have enough data
            while self.ring_count >= self.frame_size {
                self.process_frame();
                callback(&self.power_spectrum);
                self.ring_read = (self.ring_read + self.increment) % ring_len;
                self.ring_count -= self.increment;
            }
        }
    }

    fn process_frame(&mut self) {
        let ring_len = self.ring_buffer.len();

        // Apply window and convert i16 -> f32
        for i in 0..self.frame_size {
            let ring_idx = (self.ring_read + i) % ring_len;
            self.fft_input[i] = self.ring_buffer[ring_idx] as f32 * self.window[i];
        }

        // Compute real FFT
        self.fft
            .process_with_scratch(&mut self.fft_input, &mut self.fft_output, &mut self.fft_scratch)
            .expect("FFT computation failed");

        // Compute power spectrum: |X[k]|^2 = re^2 + im^2
        for (i, c) in self.fft_output.iter().enumerate() {
            self.power_spectrum[i] = c.re * c.re + c.im * c.im;
        }
    }
}
