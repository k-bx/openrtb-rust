use super::banner_ad_type::BannerAdType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Banner {
    w: u32,
    h: u32,
    btype: Vec<BannerAdType>,
}
