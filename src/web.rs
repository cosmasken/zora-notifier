use log::info;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::Filter;

pub async fn start_warp_server(data: Arc<RwLock<TypeMap>>) {
    let channel_id = {
        let data_read = data.read().await;
        let cid = data_read
            .get::<crate::ChannelIdKey>()
            .expect("ChannelIdKey not found in TypeMap")
            .clone();
        info!("Warp server retrieved channel_id Arc: {:?}", Arc::as_ptr(&cid));
        cid
    };

    let set_channel = warp::path("channelid")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_channel_id(channel_id.clone()))
        .and_then(set_channel_id);

    info!("Starting Warp server on 127.0.0.1:3030");
    warp::serve(set_channel).run(([127, 0, 0, 1], 3030)).await;
}

fn with_channel_id(
    channel_id: Arc<RwLock<Option<ChannelId>>>,
) -> impl Filter<Extract = (Arc<RwLock<Option<ChannelId>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || channel_id.clone())
}

async fn set_channel_id(
    new_channel_id: u64,
    channel_id: Arc<RwLock<Option<ChannelId>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    *channel_id.write().await = Some(ChannelId::new(new_channel_id));
    info!("Channel ID set to {} in Arc: {:?}", new_channel_id, Arc::as_ptr(&channel_id));
    Ok(warp::reply::json(&format!("Channel ID set to {}", new_channel_id)))
}