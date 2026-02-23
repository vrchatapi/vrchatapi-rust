#[derive(Debug)]
#[non_exhaustive]
pub enum File<'a> {
    Path(::std::path::PathBuf),
    Buf {
        filename: ::std::borrow::Cow<'static, str>,
        file: ::reqwest::Body,
        length: ::core::option::Option<u64>,
        mime_type: ::std::borrow::Cow<'a, str>,
    },
}

impl File<'_> {
    pub(crate) async fn get_multipart<T>(
        self,
    ) -> Result<::reqwest::multipart::Part, crate::apis::Error<T>> {
        match self {
            //This does not compile under ci (but does not under ci?!?)?
            //Self::Path(path) => Ok(reqwest::multipart::Part::file(path).await?),
            Self::Path(path) => {
                //The generator likes to generate this in 7.20.0
                let file = ::tokio::fs::File::open(&path).await?;
                let stream = ::tokio_util::codec::FramedRead::new(
                    file,
                    ::tokio_util::codec::BytesCodec::new(),
                );
                let file_name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                let file_part =
                    ::reqwest::multipart::Part::stream(::reqwest::Body::wrap_stream(stream))
                        .file_name(file_name);

                Ok(file_part)
            }
            Self::Buf {
                filename,
                file,
                length,
                mime_type,
            } => {
                let part = if let Some(length) = length {
                    ::reqwest::multipart::Part::stream_with_length(file, length)
                } else {
                    ::reqwest::multipart::Part::stream(file)
                }
                .file_name(filename)
                .mime_str(&*mime_type)?;
                Ok(part)
            }
        }
    }
}
