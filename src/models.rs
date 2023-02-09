pub struct VideoResource {
    resource_type: VideoSource, // TODO: Determine if actually needed. (Maybe just a `name` is sufficient?)
    resource_id: String,
    url_base: String,
}

enum VideoSource {
    YouTubeVideo,
    YouTubePlaylist,
    YouTubeChannel,
}

impl VideoResource {
    pub fn build(url: String) -> Option<VideoResource> {
        let result = VideoResource::determine_video_type(url);
        match result {
            Some(r) => {
                return Some(VideoResource {
                    resource_type: r.0,
                    resource_id: r.1,
                    url_base: "test".to_string(),
                })
            }
            None => None,
        }
    }

    fn determine_video_type(url: String) -> Option<(VideoSource, String)> {
        use regex::Regex;

        let resource_regex_pairs = vec![
            (
                r"^[\w-]{11}$|(?<=v=|v\\=)[\w-]{11}|(?<=youtu\.be/).{11}",
                VideoSource::YouTubeVideo,
            ),
            (r"(?<=list=)[\w\-]+", VideoSource::YouTubePlaylist),
            (
                r"(?:www\.)?youtube\.com/(?:c/|channel/|@|user/)[\w\-]+",
                VideoSource::YouTubeChannel,
            ),
        ];

        for p in resource_regex_pairs {
            let rgx = Regex::new(p.0).unwrap();
            match rgx.captures(&url) {
                Some(caps) => {
                    let cap = caps.get(0).unwrap().as_str().to_string();
                    dbg!("{}", &cap);
                    return Some((p.1, cap));
                }
                None => continue,
            }
        }

        return None;
    }

    fn full_resource_uri(&self) -> String {
        self.url_base.clone() + &self.resource_id
    }
}
