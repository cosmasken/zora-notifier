use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PreviewImage {
    pub small: String,
    pub blurhash: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaContent {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "originalUri")]
    pub original_uri: String,
    #[serde(rename = "previewImage")]
    pub preview_image: PreviewImage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Coin {
    pub name: String,
    pub description: String,
    pub address: String,
    pub symbol: String,
    #[serde(rename = "totalVolume")]
    pub total_volume: String,
    #[serde(rename = "volume24h")]
    pub volume_24h: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "marketCap")]
    pub market_cap: String,
    #[serde(rename = "marketCapDelta24h")]
    pub market_cap_delta_24h: String,
    #[serde(rename = "mediaContent")]
    pub media_content: MediaContent,
    pub transfers: Transfers,
    #[serde(rename = "uniqueHolders")]
    pub unique_holders: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transfers {
    pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Edge {
    pub node: Coin,
    pub cursor: String,
}

#[derive(Deserialize, Debug)]
pub struct PageInfo {
    #[serde(rename = "endCursor")]
    pub end_cursor: String,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
}

#[derive(Deserialize, Debug)]
pub struct ExploreList {
    pub edges: Vec<Edge>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename = "exploreList")]
    pub explore_list: ExploreList,
}