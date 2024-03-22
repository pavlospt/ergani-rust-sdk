use chrono::{NaiveDate, NaiveTime};
use ergani::{
    client::{ErganiClient, SubmissionResponse},
    models::employee::employee_announcement::EmployeeAnnouncement,
};

#[allow(dead_code)]
pub(crate) async fn submit_employee_announcement(
    ergani_client: &ErganiClient,
) -> anyhow::Result<Vec<SubmissionResponse>> {
    let birth_date = NaiveDate::parse_from_str("1990-01-01", "%Y-%m-%d")
        .unwrap()
        .format("%d/%m/%Y")
        .to_string();

    let employee_hire_date = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")
        .unwrap()
        .format("%d/%m/%Y")
        .to_string();

    let employee_hire_time = NaiveTime::parse_from_str("12:20", "%H:%M")
        .unwrap()
        .format("%H:%M")
        .to_string();

    let employee_leave_time = NaiveTime::parse_from_str("11:20", "%H:%M")
        .unwrap()
        .format("%H:%M")
        .to_string();

    let contract_from = NaiveDate::parse_from_str("2024-01-01", "%Y-%m-%d")
        .unwrap()
        .format("%d/%m/%Y")
        .to_string();

    let contract_to = NaiveDate::parse_from_str("2024-04-01", "%Y-%m-%d")
        .unwrap()
        .format("%d/%m/%Y")
        .to_string();

    let employee_announcements = vec![EmployeeAnnouncement {
        f_aa_pararthmatos: String::from("0"),
        f_rel_protocol: None,
        f_rel_date: None,
        f_ypiresia_sepe: String::from("11050"),
        f_ypiresia_oaed: String::from("186400"),
        f_ergodotikh_organwsh: None,
        f_kad_kyria: String::from("1000"),
        f_kad_deyt_1: None,
        f_kad_deyt_2: None,
        f_kad_deyt_3: None,
        f_kad_deyt_4: None,
        f_kad_pararthmatos: String::from("1005"),
        f_kallikratis_pararthmatos: String::from("00000000"),
        f_eponymo: String::from("0"),
        f_onoma: String::from("0"),
        f_eponymo_patros: None,
        f_onoma_patros: String::from("0"),
        f_eponymo_mitros: None,
        f_onoma_mitros: String::from("0"),
        f_topos_gennhshs: None,
        f_birthdate: birth_date,
        f_sex: String::from("0"),
        f_yphkoothta: String::from("000"),
        f_typos_taytothtas: String::from("ΑΑ"),
        f_ar_taytothtas: String::from("0"),
        f_ekdousa_arxh: None,
        f_date_ekdosis: None,
        f_date_ekdosis_lixi: None,
        f_res_permit_inst: None,
        f_res_permit_inst_type: None,
        f_res_permit_inst_ar: None,
        f_res_permit_inst_lixi: None,
        f_res_permit_ap: None,
        f_res_permit_ap_type: None,
        f_res_permit_ap_ar: None,
        f_res_permit_ap_lixi: None,
        f_res_permit_visa: None,
        f_res_permit_visa_ar: None,
        f_res_permit_visa_from: None,
        f_res_permit_visa_to: None,
        f_marital_status: String::from("0"),
        f_arithmos_teknon: String::from("0"),
        f_afm: String::from("123456789"),
        f_doy: None,
        f_amika: None,
        f_amka: String::from("00000000000"),
        f_code_anergias: None,
        f_ar_vivliou_anilikou: None,
        f_dieythinsi: None,
        f_kallikratis: None,
        f_tk: None,
        f_til: None,
        f_fax: None,
        f_email: None,
        f_epipedo_morfosis: String::from("0"),
        f_professional_education: None,
        f_expertise_field: None,
        f_subject_area: None,
        f_subject_group: None,
        f_education_agency: None,
        f_education_date_from: None,
        f_education_date_to: None,
        f_duration: None,
        f_education_year: None,
        f_fl1: None,
        f_fl2: None,
        f_fl3: None,
        f_fl4: None,
        f_pc: None,
        f_pc_other: None,
        f_proslipsidate: employee_hire_date,
        f_proslipsitime: employee_hire_time,
        f_apoxwrisitime: employee_leave_time,
        f_orario: None,
        f_wresexternal: None,
        f_week_hours: String::from("400,0"),
        f_orariodialeima: None,
        f_eidikothta: String::from("0"),
        f_proipiresia: String::from("0"),
        f_apodoxes: String::from("10.000,00"),
        f_hour_apodoxes: String::from("400,00"),
        f_protiergasia: String::from("0"),
        f_sxeshapasxolisis: String::from("0"),
        f_orismenou_apo: contract_from,
        f_orismenou_ews: contract_to,
        f_kathestosapasxolisis: String::from("0"),
        f_xaraktirismos: String::from("0"),
        f_special_case: None,
        f_apoalliperioxi: None,
        f_nationalityalli: None,
        f_kallikratisalli: None,
        f_responsible_position: String::from("1"),
        f_working_time_digital_organization: String::from("0"),
        f_full_employment_hours: String::from("20,0"),
        f_week_days: String::from("5"),
        f_euelikto_wrario_minutes: String::from("0"),
        f_working_card: String::from("0"),
        f_dialeimma_minutes: String::from("0"),
        f_dialeimma_entos_wrariou: String::from("0"),
        f_topothetisiepistoli: String::from("0"),
        f_topothetisioaed: String::from("0"),
        f_programaoaed: None,
        f_replaceprograma: None,
        f_replaceprograma_afm: None,
        f_replaceprograma_amka: None,
        f_epidomaoaed: String::from("0"),
        f_epidoma_ypiresia_oaed: None,
        f_sk_protocol: None,
        f_sk_date: None,
        f_comments: None,
        f_eponymo_idiotitas: String::from("0"),
        f_onoma_idiotitas: String::from("0"),
        f_idiotita_idiotitas: String::from("0"),
        f_dieythinsi_idiotitas: String::from("0"),
        f_afm_idiotitas: String::from("000000000"),
        f_afm_proswpoy: String::from("000000000"),
        f_file: None,
        f_foreign_file: None,
        f_young_file: None,
    }];

    let response = ergani_client
        .submit_employee_announcements(employee_announcements)
        .await?;

    Ok(response)
}
