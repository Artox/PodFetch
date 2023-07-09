use std::io;
use std::path::Path;
use crate::DbConnection;
use crate::models::podcast_episode::PodcastEpisode;
use crate::models::podcasts::Podcast;

use crate::service::file_service::{FileService, prepare_podcast_episode_title_to_directory};
use crate::utils::error::{CustomError, map_io_error};


pub struct PathService {}

impl PathService {
    pub fn get_podcast_episode_path(directory: &str, episode: Option<PodcastEpisode>, suffix:
    &str, filename: &str, conn: &mut DbConnection)
        -> String {
        return match episode {
            Some(episode) => {
                format!("{}/{}/podcast.{}", directory,
                        prepare_podcast_episode_title_to_directory(episode, conn), suffix)
            },
            None => {
                format!("{}/{}/podcast.{}", directory, filename, suffix)
            }
        }
    }

    pub fn get_image_path(directory: &str, episode: Option<PodcastEpisode>, _suffix: &str,
                          filename: &str,
                          conn: &mut DbConnection) -> String {
        return match episode {
            Some(episode) => {
                format!("{}/{}", directory, prepare_podcast_episode_title_to_directory(episode,
                                                                                       conn))
            },
            None => {
                format!("{}/{}", directory, filename)
            }
        }
    }

    pub fn get_image_podcast_path_with_podcast_prefix(directory: &str, suffix: &str) -> String {
        return format!("{}/image.{}", directory, suffix);
    }

    pub fn check_if_podcast_episode_directory_available(base_path:&str, podcast: Podcast,conn: &mut DbConnection) ->
                                                                                          Result<String, CustomError> {
        let mut i = 0;
        if !Path::new(&base_path).exists() {
            std::fs::create_dir(&base_path).map_err(map_io_error)?;
            return Ok(base_path.to_string());
        }

        while Path::new(&format!("{}-{}",base_path, i)).exists() {
            i += 1;
        }
        let final_path = format!("{}-{}",base_path, i);
        // This is save to insert because this directory does not exist
        std::fs::create_dir(&final_path)
            .map_err(map_io_error)?;
        return Ok(final_path);
    }
}
