use serde_derive::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OvertimeResponseRoot {
    pub title: String,
    #[serde(rename = "json")]
    pub overtimes: OvertimeJson,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OvertimeJson {
    #[serde(rename = "Overtimes")]
    pub overtimes: Overtimes,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overtimes {
    #[serde(rename = "Overtime")]
    pub overtime: Vec<Overtime>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overtime {
    #[serde(rename = "f_aa_pararthmatos")]
    pub f_aa_pararthmatos: String,
    #[serde(rename = "f_rel_protocol")]
    pub f_rel_protocol: String,
    #[serde(rename = "f_rel_date")]
    pub f_rel_date: String,
    #[serde(rename = "f_ypiresia_sepe")]
    pub f_ypiresia_sepe: String,
    #[serde(rename = "f_ergodotikh_organwsh")]
    pub f_ergodotikh_organwsh: String,
    #[serde(rename = "f_kad_kyria")]
    pub f_kad_kyria: String,
    #[serde(rename = "f_kad_deyt_1")]
    pub f_kad_deyt_1: String,
    #[serde(rename = "f_kad_deyt_2")]
    pub f_kad_deyt_2: String,
    #[serde(rename = "f_kad_deyt_3")]
    pub f_kad_deyt_3: String,
    #[serde(rename = "f_kad_deyt_4")]
    pub f_kad_deyt_4: String,
    #[serde(rename = "f_kad_pararthmatos")]
    pub f_kad_pararthmatos: String,
    #[serde(rename = "f_kallikratis_pararthmatos")]
    pub f_kallikratis_pararthmatos: String,
    #[serde(rename = "f_comments")]
    pub f_comments: String,
    #[serde(rename = "f_afm_proswpoy")]
    pub f_afm_proswpoy: String,
    #[serde(rename = "Ergazomenoi")]
    pub ergazomenoi: Ergazomenoi,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ergazomenoi {
    #[serde(rename = "OvertimeErgazomenosDate")]
    pub overtime_ergazomenos_date: Vec<OvertimeErgazomenosDate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OvertimeErgazomenosDate {
    #[serde(rename = "f_afm")]
    pub f_afm: String,
    #[serde(rename = "f_amka")]
    pub f_amka: String,
    #[serde(rename = "f_eponymo")]
    pub f_eponymo: String,
    #[serde(rename = "f_onoma")]
    pub f_onoma: String,
    #[serde(rename = "f_date")]
    pub f_date: String,
    #[serde(rename = "f_from")]
    pub f_from: String,
    #[serde(rename = "f_to")]
    pub f_to: String,
    #[serde(rename = "f_from_2")]
    pub f_from_2: String,
    #[serde(rename = "f_to_2")]
    pub f_to_2: String,
    #[serde(rename = "f_cancellation")]
    pub f_cancellation: String,
    #[serde(rename = "f_step")]
    pub f_step: String,
    #[serde(rename = "f_reason")]
    pub f_reason: String,
    #[serde(rename = "f_weekdates")]
    pub f_weekdates: String,
    #[serde(rename = "f_asee")]
    pub f_asee: String,
}
