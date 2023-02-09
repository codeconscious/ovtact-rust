use std::fmt;

pub enum VideoType {
    YouTubeVideo,
    YouTubePlaylist,
    YouTubeChannel,
}

impl fmt::Display for VideoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            VideoType::YouTubeVideo => "YouTube video",
            VideoType::YouTubePlaylist => "YouTube playlist",
            VideoType::YouTubeChannel => "YouTube channel",
        };
        write!(f, "{}", text)
    }
}

pub struct VideoResource {
    pub resource_type: VideoType,
    pub resource_id: String,
    pub url_base: String,
}

impl VideoResource {
    pub fn build(url: String) -> Result<VideoResource, String> {
        let result = VideoResource::determine_video_type(url);
        match result {
            Some(r) => {
                return Ok(VideoResource {
                    resource_type: r.0,
                    resource_id: r.1,
                    url_base: "test".to_string(),
                })
            }
            None => Err("Invalid URL.".to_string()),
        }
    }

    fn determine_video_type(url: String) -> Option<(VideoType, String)> {
        let resource_regex_pairs = vec![
            (
                r"^[\w-]{11}$|(?<=v=|v\\=)[\w-]{11}|(?<=youtu\.be/).{11}",
                VideoType::YouTubeVideo,
            ),
            (r"(?<=list=)[\w\-]+", VideoType::YouTubePlaylist),
            (
                r"(?:www\.)?youtube\.com/(?:c/|channel/|@|user/)[\w\-]+",
                VideoType::YouTubeChannel,
            ),
        ];

        for p in resource_regex_pairs {
            let rgx = onig::Regex::new(p.0).unwrap();
            match rgx.captures(&url) {
                Some(capts) => {
                    let position_pair = capts.pos(0);
                    let matched_text = match position_pair {
                        Some(v) => &url[v.0..v.1],
                        None => panic!("Unexpected regex error")
                    };

                    // println!("Matched text: {}", matched_text);
                    return Some((p.1, matched_text.to_string()));
                }
                None => continue,
            }
        }

        return None;
    }

    pub fn full_resource_uri(&self) -> String {
        self.url_base.clone() + &self.resource_id
    }
}

#[derive(Debug)]
pub struct VideoDownloadError {
    pub message: String,
    pub inner: std::io::Error,
}
