use crate::config::environment::Config;
use std::path::Path;
use std::time::Instant;
use uuid::Uuid;
use cloudinary::upload::{Upload, Source, OptionalParameters};
use std::collections::BTreeSet;

pub struct CloudinaryService {
    upload: Upload,
    config: Config,
}

impl CloudinaryService {
    pub fn new(config: Config) -> Self {
        let upload = Upload::new(
            config.cloudinary_api_key.clone(),
            config.cloudinary_cloud_name.clone(),
            config.cloudinary_api_secret.clone(),
        );
        Self { upload, config }
    }

    pub fn generate_public_id(&self, _filename: &str) -> String {
        format!("ocr_{}", Uuid::new_v4())
    }

    pub async fn upload_image(&self, file_path: &Path, public_id: &str) -> Result<String, String> {
        let start_time = Instant::now();
        
        let options = BTreeSet::from([OptionalParameters::PublicId(public_id.to_string())]);

        match self.upload.image(Source::Path(file_path.to_path_buf()), &options).await {
            Ok(_response) => {
                let _processing_time = start_time.elapsed().as_millis() as u64;
                let secure_url = format!(
                    "https://res.cloudinary.com/{}/image/upload/{}",
                    self.config.cloudinary_cloud_name,
                    public_id
                );
                Ok(secure_url)
            }
            Err(e) => {
                 Err(format!("Failed to upload image: {}", e))
            }
        }
    }

    pub async fn delete_image(&self, public_id: &str) -> Result<(), String> {
        match self.upload.destroy(public_id).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete image: {}", e))
        }
    }
} 