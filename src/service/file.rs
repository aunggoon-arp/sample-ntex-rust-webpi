use std::io::Write;
use futures::{StreamExt, TryStreamExt};
use ntex::{web, web::HttpResponse};
use ntex_multipart::Multipart;
use uuid::Uuid;

use crate::error::CustomError;

pub struct FileService;

impl FileService {
    pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse,  CustomError> {
        while let Ok(Some(mut field)) = payload.try_next().await {
            let filename = Uuid::new_v4();
            let filepath = format!("./upload/{}", filename);
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
            }
        }
        Ok(HttpResponse::Ok().into())
    }
}
