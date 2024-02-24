use crate::models::company::company_overtime::CompanyOvertime;
use crate::models::overtime::Overtime;
use chrono::NaiveDate;

#[derive(Default)]
pub struct CompanyOvertimeBuilder {
    pub(crate) business_branch_number: i64,
    pub(crate) sepe_service_code: String,
    pub(crate) business_primary_activity_code: String,
    pub(crate) business_branch_activity_code: String,
    pub(crate) kallikratis_municipal_code: String,
    pub(crate) legal_representative_tax_identification_number: String,
    pub(crate) employee_overtimes: Vec<Overtime>,
    pub(crate) related_protocol_id: Option<String>,
    pub(crate) related_protocol_date: Option<NaiveDate>,
    pub(crate) employer_organization: Option<String>,
    pub(crate) business_secondary_activity_code_1: Option<String>,
    pub(crate) business_secondary_activity_code_2: Option<String>,
    pub(crate) business_secondary_activity_code_3: Option<String>,
    pub(crate) business_secondary_activity_code_4: Option<String>,
    pub(crate) comments: Option<String>,
}

impl CompanyOvertimeBuilder {
    pub fn builder() -> CompanyOvertimeBuilder {
        CompanyOvertimeBuilder::default()
    }

    pub fn build(self) -> CompanyOvertime {
        CompanyOvertime {
            business_branch_number: self.business_branch_number,
            sepe_service_code: self.sepe_service_code,
            business_primary_activity_code: self.business_primary_activity_code,
            business_branch_activity_code: self.business_branch_activity_code,
            kallikratis_municipal_code: self.kallikratis_municipal_code,
            legal_representative_tax_identification_number: self
                .legal_representative_tax_identification_number,
            employee_overtimes: self.employee_overtimes,
            related_protocol_id: self.related_protocol_id,
            related_protocol_date: self.related_protocol_date,
            employer_organization: self.employer_organization,
            business_secondary_activity_code_1: self.business_secondary_activity_code_1,
            business_secondary_activity_code_2: self.business_secondary_activity_code_2,
            business_secondary_activity_code_3: self.business_secondary_activity_code_3,
            business_secondary_activity_code_4: self.business_secondary_activity_code_4,
            comments: self.comments,
        }
    }

    pub fn set_business_branch_number(mut self, business_branch_number: i64) -> Self {
        self.business_branch_number = business_branch_number;
        self
    }
    pub fn set_sepe_service_code(mut self, sepe_service_code: impl Into<String>) -> Self {
        self.sepe_service_code = sepe_service_code.into();
        self
    }
    pub fn set_business_primary_activity_code(
        mut self,
        business_primary_activity_code: impl Into<String>,
    ) -> Self {
        self.business_primary_activity_code = business_primary_activity_code.into();
        self
    }
    pub fn set_business_branch_activity_code(
        mut self,
        business_branch_activity_code: impl Into<String>,
    ) -> Self {
        self.business_branch_activity_code = business_branch_activity_code.into();
        self
    }
    pub fn set_kallikratis_municipal_code(
        mut self,
        kallikratis_municipal_code: impl Into<String>,
    ) -> Self {
        self.kallikratis_municipal_code = kallikratis_municipal_code.into();
        self
    }
    pub fn set_legal_representative_tax_identification_number(
        mut self,
        legal_representative_tax_identification_number: impl Into<String>,
    ) -> Self {
        self.legal_representative_tax_identification_number =
            legal_representative_tax_identification_number.into();
        self
    }
    pub fn set_employee_overtimes(mut self, employee_overtimes: Vec<Overtime>) -> Self {
        self.employee_overtimes = employee_overtimes;
        self
    }
    pub fn set_related_protocol_id(
        mut self,
        related_protocol_id: Option<impl Into<String>>,
    ) -> Self {
        self.related_protocol_id = related_protocol_id.map(|s| s.into());
        self
    }
    pub fn set_related_protocol_date(mut self, related_protocol_date: Option<NaiveDate>) -> Self {
        self.related_protocol_date = related_protocol_date;
        self
    }
    pub fn set_employer_organization(
        mut self,
        employer_organization: Option<impl Into<String>>,
    ) -> Self {
        self.employer_organization = employer_organization.map(|s| s.into());
        self
    }
    pub fn set_business_secondary_activity_code_1(
        mut self,
        business_secondary_activity_code_1: Option<impl Into<String>>,
    ) -> Self {
        self.business_secondary_activity_code_1 =
            business_secondary_activity_code_1.map(|s| s.into());
        self
    }
    pub fn set_business_secondary_activity_code_2(
        mut self,
        business_secondary_activity_code_2: Option<impl Into<String>>,
    ) -> Self {
        self.business_secondary_activity_code_2 =
            business_secondary_activity_code_2.map(|s| s.into());
        self
    }
    pub fn set_business_secondary_activity_code_3(
        mut self,
        business_secondary_activity_code_3: Option<impl Into<String>>,
    ) -> Self {
        self.business_secondary_activity_code_3 =
            business_secondary_activity_code_3.map(|s| s.into());
        self
    }
    pub fn set_business_secondary_activity_code_4(
        mut self,
        business_secondary_activity_code_4: Option<impl Into<String>>,
    ) -> Self {
        self.business_secondary_activity_code_4 =
            business_secondary_activity_code_4.map(|s| s.into());
        self
    }
    pub fn set_comments(mut self, comments: Option<impl Into<String>>) -> Self {
        self.comments = comments.map(|s| s.into());
        self
    }
}
