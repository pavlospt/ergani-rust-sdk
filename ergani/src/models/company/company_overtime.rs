use crate::internal::utils::format_date;
use crate::models::overtime::Overtime;
use chrono::NaiveDate;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Serialize as TypeSerialize;

/// Represents overtime entries that are issued on a single business branch
/// * - `business_branch_number` - The number identifying the specific business branch
/// * - `sepe_service_code` - The SEPE service code
/// * - `business_primary_activity_code` - The primary activity code of the business
/// * - `business_branch_activity_code` - The activity code for the specific branch
/// * - `kallikratis_municipal_code` - The kallikratis municipal code
/// * - `legal_representative_tax_identification_number` - Tax identification number of the legal representative
/// * - `employee_overtimes` - A list of `Overtime` entries for employees
/// * - `related_protocol_id` - Related protocol ID
/// * - `related_protocol_date` - The date of the related protocol
/// * - `employer_organization` - The employer's organization name
/// * - `business_secondary_activity_code_1` - Secondary activity code 1
/// * - `business_secondary_activity_code_2` - Secondary activity code 2
/// * - `business_secondary_activity_code_3` - Secondary activity code 3
/// * - `business_secondary_activity_code_4` - Secondary activity code 4
/// * - `comments` - Additional comments related to the overtime entries
pub struct CompanyOvertime {
    pub business_branch_number: i64,
    pub sepe_service_code: String,
    pub business_primary_activity_code: String,
    pub business_branch_activity_code: String,
    pub kallikratis_municipal_code: String,
    pub legal_representative_tax_identification_number: String,
    pub employee_overtimes: Vec<Overtime>,
    pub related_protocol_id: Option<String>,
    pub related_protocol_date: Option<NaiveDate>,
    pub employer_organization: Option<String>,
    pub business_secondary_activity_code_1: Option<String>,
    pub business_secondary_activity_code_2: Option<String>,
    pub business_secondary_activity_code_3: Option<String>,
    pub business_secondary_activity_code_4: Option<String>,
    pub comments: Option<String>,
}

#[derive(TypeSerialize)]
struct CompanyOvertimes {
    #[serde(rename = "OvertimeErgazomenosDate")]
    employee_overtimes: Vec<Overtime>,
}

impl Serialize for CompanyOvertime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let related_protocol_id = match &self.related_protocol_id {
            Some(id) => id,
            None => "",
        }
        .to_string();

        let related_protocol_date = match &self.related_protocol_date {
            Some(date) => format_date(Some(date)).to_string(),
            None => "".to_string(),
        };

        let employer_organization = match &self.employer_organization {
            Some(organization) => organization.to_string(),
            None => "".to_string(),
        };

        let business_secondary_activity_code_1 = match &self.business_secondary_activity_code_1 {
            Some(code) => code,
            None => "",
        }
        .to_string();

        let business_secondary_activity_code_2 = match &self.business_secondary_activity_code_2 {
            Some(code) => code,
            None => "",
        }
        .to_string();

        let business_secondary_activity_code_3 = match &self.business_secondary_activity_code_3 {
            Some(code) => code,
            None => "",
        }
        .to_string();

        let business_secondary_activity_code_4 = match &self.business_secondary_activity_code_4 {
            Some(code) => code,
            None => "",
        }
        .to_string();

        let comments = match &self.comments {
            Some(comments) => comments,
            None => "",
        }
        .to_string();

        let overtimes = CompanyOvertimes {
            employee_overtimes: self.employee_overtimes.clone(),
        };

        let mut company_overtime = serializer.serialize_struct("CompanyOvertime", 15)?;
        company_overtime.serialize_field("f_aa_pararthmatos", &self.business_branch_number)?;
        company_overtime.serialize_field("f_ypiresia_sepe", &self.sepe_service_code)?;
        company_overtime.serialize_field("f_kad_kyria", &self.business_primary_activity_code)?;
        company_overtime
            .serialize_field("f_kad_pararthmatos", &self.business_branch_activity_code)?;
        company_overtime.serialize_field(
            "f_kallikratis_pararthmatos",
            &self.kallikratis_municipal_code,
        )?;
        company_overtime.serialize_field(
            "f_afm_proswpoy",
            &self.legal_representative_tax_identification_number,
        )?;
        company_overtime.serialize_field("f_rel_protocol", &related_protocol_id)?;
        company_overtime.serialize_field("f_rel_date", &related_protocol_date)?;
        company_overtime.serialize_field("f_ergodotikh_organwsh", &employer_organization)?;
        company_overtime.serialize_field("f_kad_deyt_1", &business_secondary_activity_code_1)?;
        company_overtime.serialize_field("f_kad_deyt_2", &business_secondary_activity_code_2)?;
        company_overtime.serialize_field("f_kad_deyt_3", &business_secondary_activity_code_3)?;
        company_overtime.serialize_field("f_kad_deyt_4", &business_secondary_activity_code_4)?;
        company_overtime.serialize_field("f_comments", &comments)?;
        company_overtime.serialize_field("Ergazomenoi", &overtimes)?;
        company_overtime.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::overtime_justification_type::OvertimeJustificationType;
    use crate::models::weekly_work_days::WeeklyWorkDays;
    use chrono::{DateTime, NaiveDate, Utc};

    #[test]
    fn test_serialize_company_overtime() {
        let date_time_start_text = "2014-11-28T12:00:09Z";
        let date_time_end_text = "2014-11-29T12:00:09Z";

        let dt_start = date_time_start_text.parse::<DateTime<Utc>>().unwrap();
        let dt_end = date_time_end_text.parse::<DateTime<Utc>>().unwrap();

        let overtime = Overtime {
            employee_tax_identification_number: "123456789".to_string(),
            employee_social_security_number: "12345678901".to_string(),
            employee_last_name: "ΠΑΠΑΔΟΠΟΥΛΟΣ".to_string(),
            employee_first_name: "ΓΕΩΡΓΙΟΣ".to_string(),
            overtime_date: NaiveDate::from_ymd_opt(2021, 1, 1).unwrap(),
            overtime_start_time: dt_start,
            overtime_end_time: dt_end,
            overtime_cancellation: false,
            employee_profession_code: "1234".to_string(),
            overtime_justification:
                OvertimeJustificationType::AccidentPreventionOrDamageRestoration,
            weekly_workdays_number: WeeklyWorkDays::Five,
            asee_approval: Some("123456".to_string()),
        };

        let company_overtime = CompanyOvertime {
            business_branch_number: 1,
            sepe_service_code: "123456".to_string(),
            business_primary_activity_code: "1234".to_string(),
            business_branch_activity_code: "1234".to_string(),
            kallikratis_municipal_code: "1234".to_string(),
            legal_representative_tax_identification_number: "123456789".to_string(),
            employee_overtimes: vec![overtime],
            related_protocol_id: Some("123456".to_string()),
            related_protocol_date: Some(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap()),
            employer_organization: Some("".to_string()),
            business_secondary_activity_code_1: None,
            business_secondary_activity_code_2: None,
            business_secondary_activity_code_3: None,
            business_secondary_activity_code_4: None,
            comments: None,
        };

        let serialized = serde_json::to_string(&company_overtime).unwrap();
        let expected_text = crate::tests::load_fixture_as_text("company_overtime_fixture.json");
        assert_eq!(serialized, expected_text);
    }
}
