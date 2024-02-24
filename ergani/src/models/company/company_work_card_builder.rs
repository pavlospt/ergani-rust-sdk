#![allow(dead_code)]

use crate::models::company::company_work_card::CompanyWorkCard;
use crate::models::work_card::WorkCard;

#[derive(Default)]
pub struct CompanyWorkCardBuilder {
    pub(crate) employer_tax_identification_number: String,
    pub(crate) business_branch_number: i64,
    pub(crate) comments: Option<String>,
    pub(crate) card_details: Vec<WorkCard>,
}

impl CompanyWorkCardBuilder {
    pub fn builder() -> CompanyWorkCardBuilder {
        CompanyWorkCardBuilder::default()
    }

    pub fn build(self) -> CompanyWorkCard {
        CompanyWorkCard {
            employer_tax_identification_number: self.employer_tax_identification_number,
            business_branch_number: self.business_branch_number,
            comments: self.comments,
            card_details: self.card_details,
        }
    }

    pub fn set_employer_tax_identification_number(
        mut self,
        employer_tax_identification_number: impl Into<String>,
    ) -> Self {
        self.employer_tax_identification_number = employer_tax_identification_number.into();
        self
    }
    pub fn set_business_branch_number(mut self, business_branch_number: i64) -> Self {
        self.business_branch_number = business_branch_number;
        self
    }
    pub fn set_comments(mut self, comments: Option<impl Into<String>>) -> Self {
        self.comments = comments.map(|s| s.into());
        self
    }
    pub fn set_card_details(mut self, card_details: Vec<WorkCard>) -> Self {
        self.card_details = card_details;
        self
    }
}
