use alloy_primitives::BlockNumber;
use futures::Stream;
use reth_network_p2p::{
    bodies::{downloader::BodyDownloader, response::BlockResponse},
    error::{DownloadError, DownloadResult},
};
use reth_primitives::BlockBody;
use std::ops::RangeInclusive;

/// A [`BodyDownloader`] implementation that does nothing.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct NoopBodiesDownloader;

impl BodyDownloader for NoopBodiesDownloader {
    type Body = BlockBody;

    fn set_download_range(&mut self, _: RangeInclusive<BlockNumber>) -> DownloadResult<()> {
        Ok(())
    }
}

impl Stream for NoopBodiesDownloader {
    type Item = Result<Vec<BlockResponse<alloy_consensus::Header, BlockBody>>, DownloadError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        panic!("NoopBodiesDownloader shouldn't be polled.")
    }
}
