use serde_derive::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct WorkCardResponseRoot {
    pub title: String,
    #[serde(rename = "json")]
    pub work_cards: WorkCardJson,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkCardJson {
    #[serde(rename = "Cards")]
    pub cards: WorkCardCards,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkCardCards {
    #[serde(rename = "Card")]
    pub card: Vec<WorkCardCard>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkCardCard {
    #[serde(rename = "f_afm_ergodoti")]
    pub f_afm_ergodoti: String,
    #[serde(rename = "f_aa")]
    pub f_aa: String,
    #[serde(rename = "f_comments")]
    pub f_comments: String,
    #[serde(rename = "Details")]
    pub details: WorkCardDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkCardDetails {
    #[serde(rename = "CardDetails")]
    pub card_details: Vec<WorkCardCardDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkCardCardDetail {
    #[serde(rename = "f_afm")]
    pub f_afm: String,
    #[serde(rename = "f_eponymo")]
    pub f_eponymo: String,
    #[serde(rename = "f_onoma")]
    pub f_onoma: String,
    #[serde(rename = "f_type")]
    pub f_type: String,
    #[serde(rename = "f_reference_date")]
    pub f_reference_date: String,
    #[serde(rename = "f_date")]
    pub f_date: String,
    #[serde(rename = "f_aitiologia")]
    pub f_aitiologia: String,
}
