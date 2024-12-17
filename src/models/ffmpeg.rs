use crate::misc::schema_generated::ws_api::MediaSources;
use log::{debug, warn};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};

#[allow(clippy::upper_case_acronyms)]
pub struct FFMPEG {
    pub file_path: String,
    process: Option<Child>,
}

impl FFMPEG {
    pub fn new(file_path: String) -> Self {
        FFMPEG { file_path, process: None }
    }

    fn format_sdp(media_sources: MediaSources) -> Result<String, String> {
        let audio_rtps = media_sources.audio();
        let mut sdp: Vec<String> =
            vec!["v=0 o=- 0 0 IN IP4 127.0.0.1 s=FFmpeg c=IN IP4 127.0.0.1 t=0 0".to_string()];

        for audio_rtp in audio_rtps {
            sdp.push(format!("m=audio {} RTP/AVP {}", audio_rtp.port(), audio_rtp.payload_type()));
            sdp.push(format!(
                "a=rtpmap:{} {}/{}",
                audio_rtp.payload_type(),
                audio_rtp.codec(),
                audio_rtp.clock_rate()
            ));
            sdp.push("a=sendonly".to_string());
        }
        sdp.push(format!(
            "-c:a aac -b:a 128k -ac 2 -filter_complex amerge=inputs={}",
            audio_rtps.len()
        ));
        let camera_rtps = media_sources.camera();
        if !camera_rtps.is_empty() {
            let layout = match camera_rtps.len() {
                1 => format!("a=filter:complex [0:v]drawtext=text='{}':x=10:y=h-30[v0]; -map [v0]", camera_rtps.get(0).label()),
                2 => format!("a=filter:complex [0:v]drawtext=text='{}':x=10:y=h-30[v0];[1:v]drawtext=text='{}':x=10:y=h-30[v1];[v0][v1]hstack=inputs=2[v]; -map [v]", camera_rtps.get(0).label(), camera_rtps.get(1).label()),
                3 => format!("a=filter:complex [0:v]drawtext=text='{}':x=10:y=h-30[v0];[1:v]drawtext=text='{}':x=10:y=h-30[v1];[v0][v1]hstack=inputs=2[top];[2:v]drawtext=text='{}':x=10:y=h-30[v2];[top][v2]vstack=inputs=2[v]; -map [v]", camera_rtps.get(0).label(), camera_rtps.get(1).label(), camera_rtps.get(2).label()),
                4 => format!("a=filter:complex [0:v]drawtext=text='{}':x=10:y=h-30[v0];[1:v]drawtext=text='{}':x=10:y=h-30[v1];[v0][v1]hstack=inputs=2[top];[2:v]drawtext=text='{}':x=10:y=h-30[v2];[3:v]drawtext=text='{}':x=10:y=h-30[v3];[v2][v3]hstack=inputs=2[bottom];[top][bottom]vstack=inputs=2[v]; -map [v]", camera_rtps.get(0).label(), camera_rtps.get(1).label(), camera_rtps.get(2).label(), camera_rtps.get(3).label()),
                _ => return Err("Unsupported number of camera RTPs".to_string()),
            };

            for video_rtp in camera_rtps {
                sdp.push(format!(
                    "m=video {} RTP/AVP {}",
                    video_rtp.port(),
                    video_rtp.payload_type()
                ));
                sdp.push(format!(
                    "a=rtpmap:{} {}/{}",
                    video_rtp.payload_type(),
                    video_rtp.codec(),
                    video_rtp.clock_rate()
                ));
                sdp.push("a=sendonly".to_string());
            }

            sdp.push(layout);
            sdp.push("-c:v libx264".to_string());
        }
        Ok(sdp.join("\n"))
    }

    pub async fn merge(
        &mut self,
        media_sources: MediaSources<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sdp = Self::format_sdp(media_sources)?;
        #[rustfmt::skip] // to keep option and value on the same line
        let args = vec![
            "-protocol_whitelist", "pipe,udp,rtp",
            "-fflags", "+genpts",
            "-f", "sdp",
            "-i", "pipe:0",
            "-vf", "scale=1280:720",
            "-r", "30",
            "-f", "mp4",
            &self.file_path,
        ];

        let mut process = Command::new("ffmpeg")
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = process.stdin.take() {
            stdin.write_all(sdp.as_bytes()).await?;
            stdin.shutdown().await?;
        }

        let pid = process.id();
        self.process = Some(process);
        debug!("FFMPEG process (pid:{:?}) spawned, outputting to {}", pid, self.file_path);
        Ok(())
    }

    pub async fn kill(&mut self) {
        if let Some(process) = &mut self.process {
            if let Err(e) = process.kill().await {
                warn!("Failed to kill FFMPEG process: {:?}", e);
            }
        }
    }
}
