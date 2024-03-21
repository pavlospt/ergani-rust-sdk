use serde_derive::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WeekScheduleResponseRoot {
    pub title: String,
    #[serde(rename = "json")]
    pub week_schedule: WeekScheduleJson,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WeekScheduleJson {
    #[serde(rename = "WTOS")]
    pub wtos: WeekScheduleWtos,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WeekScheduleWtos {
    #[serde(rename = "WTO")]
    pub wto: Vec<WeekScheduleWto>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekScheduleWto {
    #[serde(rename = "f_aa_pararthmatos")]
    pub f_aa_pararthmatos: String,
    #[serde(rename = "f_rel_protocol")]
    pub f_rel_protocol: String,
    #[serde(rename = "f_rel_date")]
    pub f_rel_date: String,
    #[serde(rename = "f_comments")]
    pub f_comments: String,
    #[serde(rename = "f_from_date")]
    pub f_from_date: String,
    #[serde(rename = "f_to_date")]
    pub f_to_date: String,
    #[serde(rename = "Ergazomenoi")]
    pub ergazomenoi: WeekScheduleErgazomenoi,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekScheduleErgazomenoi {
    #[serde(rename = "ErgazomenoiWTO")]
    pub ergazomenoi_wto: Vec<WeekScheduleErgazomenoiWto>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekScheduleErgazomenoiWto {
    #[serde(rename = "f_afm")]
    pub f_afm: String,
    #[serde(rename = "f_eponymo")]
    pub f_eponymo: String,
    #[serde(rename = "f_onoma")]
    pub f_onoma: String,
    #[serde(rename = "f_day")]
    pub f_day: Option<String>,
    #[serde(rename = "ErgazomenosAnalytics")]
    pub ergazomenos_analytics: WeekScheduleErgazomenosAnalytics,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekScheduleErgazomenosAnalytics {
    #[serde(rename = "ErgazomenosWTOAnalytics")]
    pub ergazomenos_wtoanalytics: Vec<WeekScheduleErgazomenosWtoanalytic>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekScheduleErgazomenosWtoanalytic {
    #[serde(rename = "f_type")]
    pub f_type: String,
    #[serde(rename = "f_from")]
    pub f_from: String,
    #[serde(rename = "f_to")]
    pub f_to: String,
}
