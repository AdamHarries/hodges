extern crate stft;
use stft::{STFT, WindowType};

use std::f32;

// the stft from librosa, re-implemented in rust
fn stft(
    y: Vec<f32>,
    n_fft_o: Option<i32>,
    hop_length_o: Option<i32>,
    win_length_o: Option<i32>,
    center_o: Option<bool>,
) {
    /** Short-time Fourier transform (STFT)
     * Parameters:
     *
     * y : real valued input signal (audio time series)
     * n_fft : fft window size
     * hop_length : number of audio frames between stft columns.
     *     If unspecified, defaults to win_length/4
     * win_length: each frame of audio is windowed by window().
     *     The window will be of length win_length, and then padded with zeros to match n_fft.
     *     If unspecified, defaults to win_length = n_fft.
     * center:
     *     if True, the signal 'y' is padded so that the frame D[:, t] is centered at y[t*hop_length].
     *     if False, then D[:,t] begins at y[t*hop_length]
     */
    let n_fft = n_fft_o.unwrap_or(2048);
    // By default, use the entire frame.
    let win_length = win_length_o.unwrap_or(n_fft);

    let hop_length = hop_length_o.unwrap_or(win_length / 4);

    let wl = match win_length {
        Some(i) => i,
        None => n_fft,
    };

    // fft_window = get_window(window, win_length, fftbins=True)

    // Pad the window out to n_fft size
    // fft_window = util.pad_center(fft_window, n_fft)

    // Reshape so that the window can be broadcast
    // fft_window = fft_window.reshape((-1, 1))

    //      Check audio is valid
    //     util.valid_audio(y)

    //  Pad the time series so that frames are centered
    //     if center:
    //         y = np.pad(y, int(n_fft // 2), mode=pad_mode)

    //  Window the time series.
    //     y_frames = util.frame(y, frame_length=n_fft, hop_length=hop_length)

    //  Pre-allocate the STFT matrix
    //     stft_matrix = np.empty((int(1 + n_fft // 2), y_frames.shape[1]),
    //                            dtype=dtype,
    //                            order='F')

    //  how many columns can we fit within MAX_MEM_BLOCK?
    //     n_columns = int(util.MAX_MEM_BLOCK / (stft_matrix.shape[0] *
    //                                           stft_matrix.itemsize))

    //     for bl_s in range(0, stft_matrix.shape[1], n_columns):
    //         bl_t = min(bl_s + n_columns, stft_matrix.shape[1])

    //      RFFT and Conjugate here to match phase from DPWE code
    //         stft_matrix[:, bl_s:bl_t] = fft.fft(fft_window *
    //                                             y_frames[:, bl_s:bl_t],
    //                                             axis=0)[:stft_matrix.shape[0]]

    //     return stft_matrix
}

fn main() {
    let mut input: Vec<Complex<f32>> = vec![Complex::zero(); 4096];
    for (i, t) in input.iter_mut().enumerate() {
        t.re = (i as f32 / 4096.0).sin();
    }

    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 4096];

    let fft = Radix4::new(4096, false);
    fft.process(&mut input, &mut output);

    println!("{:?}", output);
}
