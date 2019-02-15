// This mod does not expose anything right now
// In future, for performance reasons, we may want to expose the audiostream + audiobuffer

// pub fn f_fr_ia(filename: String, estimator: &mut SimpleEstimator) -> f32 {
//     let mut command = Command::new("ffmpeg");

//     let args: Vec<String> = vec![
//         "-loglevel",
//         "quiet",
//         // the first ffmpeg argument is the input filename
//         "-i",
//         filename.as_str(),
//         // next, the format of the output,
//         "-f",
//         "f32le",
//         // then the codec
//         "-acodec",
//         "pcm_f32le",
//         // then the number of channels in the output
//         "-ac",
//         "1",
//         //  our output sample rate
//         "-ar",
//         "44100",
//         // finally, tell ffmpeg to write to stdout
//         "pipe:1",
//     ]
//     .iter()
//     .map(|s| s.to_string())
//     .collect();

//     command.args(args);

//     let mut child = command
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to spawn ffmpeg child process");

//     let buffer = match &mut child.stdout {
//         Some(s) => Some(audiobuffer::AudioBuffer::from_stream(s)),
//         None => panic!("Failed to get stdio from child stream!"),
//     }
//     .expect("Failed to construct buffer!");

//     child.wait().expect("Failed to wait on ffmpeg child call!");

//     // DO SOMETHING WITH THE BUFFER HERE
// }

// pub fn f_fr(filename: String) -> ()) {
//     let mut command = Command::new("ffmpeg");

//     let args: Vec<String> = vec![
//         "-loglevel",
//         "quiet",
//         // the first ffmpeg argument is the input filename
//         "-i",
//         filename.as_str(),
//         // next, the format of the output,
//         "-f",
//         "f32le",
//         // then the codec
//         "-acodec",
//         "pcm_f32le",
//         // then the number of channels in the output
//         "-ac",
//         "1",
//         //  our output sample rate
//         "-ar",
//         "44100",
//         // finally, tell ffmpeg to write to stdout
//         "pipe:1",
//     ]
//     .iter()
//     .map(|s| s.to_string())
//     .collect();

//     command.args(args);

//     let mut child = command
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to spawn ffmpeg child process");

    

//     let result = {
//         let stream = match &mut child.stdout {
//             Some(s) => Some(audiostream::AudioStream::from_stream(s)),
//             None => panic!("Failed to get stdio from child stream!"),
//         }
//         .expect("Failed to construct stream!");

//         // DO SOMETHING WITH THE STREAM HERE
//     };

//     child.wait().expect("Failed to wait on ffmpeg child call!");

// }