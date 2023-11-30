use serde::Serialize;

/// Information about an uploaded file.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadedFile {
    pub filename: String,
    pub hash: String,

    pub s3_key: String,
    pub s3_url: String,

    pub mime_type: String,

    pub bucket_name: String,

    pub size: usize,
}

impl UploadedFile {
    /// Construct new uploaded file info container.
    pub fn new(
        filename: impl Into<String>,
        s3_key: impl Into<String>,
        s3_url: impl Into<String>,

        hash: impl Into<String>,
        bucket_name: impl Into<String>,

        mime_type: impl Into<String>,
        size: usize,
    ) -> Self {
        Self {
            filename: filename.into(),
            s3_key: s3_key.into(),
            s3_url: s3_url.into(),

            hash: hash.into(),
            bucket_name: bucket_name.into(),

            mime_type: mime_type.into(),
            size,
        }
    }
}
