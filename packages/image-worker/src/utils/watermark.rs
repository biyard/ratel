use std::sync::Arc;

use base64::{Engine, engine::general_purpose};
use dto::{Artwork, ArtworkRepositoryUpdateRequest, Error, File, Result, reqwest, sqlx::PgPool};

use crate::utils::s3_client::{PresignedUrl, S3Client};

pub async fn process_watermark_async(
    pool: &PgPool,
    s3_client: Arc<S3Client>,
    artwork_id: i64,
    original_url: String,
) -> Result<()> {
    let bytes = read_image_from_url(&original_url).await?;
    let watermarked_bytes = visible_watermarking(bytes)?;

    let PresignedUrl {
        presigned_uris,
        uris,
    } = s3_client.get_put_object_uri(1).await?;

    let client = reqwest::Client::new();
    client
        .put(presigned_uris[0].clone())
        .body(watermarked_bytes)
        .send()
        .await?;

    Artwork::get_repository(pool.clone())
        .update(
            artwork_id,
            ArtworkRepositoryUpdateRequest {
                file: Some(File {
                    url: Some(uris[0].clone()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?;

    Ok(())
}

async fn read_image_from_url(url: &str) -> Result<Vec<u8>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let bytes = client.get(url).send().await?.bytes().await?;
    Ok(bytes.to_vec())
}

fn visible_watermarking(slice: Vec<u8>) -> Result<Vec<u8>> {
    let mut orig = photon_rs::PhotonImage::new_from_byteslice(slice);

    let width = orig.get_width();
    let height = orig.get_height();
    if width == 0 || height == 0 {
        tracing::error!("Image has zero width or height");
        return Err(Error::BadRequest);
    }
    // Update the path below to the correct location of your watermark image file
    let logo = include_bytes!("./protected.png");

    let logo = general_purpose::STANDARD.encode(logo.as_ref());

    let wm = photon_rs::PhotonImage::new_from_base64(&logo);

    let wm_width = wm.get_width();
    let wm_height = wm.get_height();

    let w_width = (width as f32 * 0.8) as u32;
    let w_height = ((w_width as f32 / wm_width as f32) * wm_height as f32) as u32;
    let wm = photon_rs::transform::resize(
        &wm,
        w_width,
        w_height,
        photon_rs::transform::SamplingFilter::Nearest,
    );
    let x = (width - w_width) / 2;
    let y = (height - w_height) / 2;

    photon_rs::multiple::watermark(&mut orig, &wm, x as i64, y as i64);
    let bytes = orig.get_bytes();

    Ok(bytes)
}
