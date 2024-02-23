# Ergani Rust SDK

`ergani` is a Rust SDK for interacting with the API
of [Ergani](https://www.gov.gr/en/ipiresies/ergasia-kai-asphalise/apozemioseis-kai-parokhes/prosopopoiemene-plerophorese-misthotou-ergane).

# Attributions
Ergani Rust SDK is a Rust flavour of the relevant [Ergani Python SDK](https://github.com/withlogicco/ergani-python-sdk)
built by [WITHLOGICO](https://withlogic.co/).

## Requirements

Rust 1.76.0 or later

## Installation

The `ergani` package is available on crates.io and you can install it through your favorite package manager:

```bash
cargo install ergani
```

## Usage

### Create a client

To create a new Ergani client you have to set your Ergani username, password and optionally the Ergani API base URL,
that defaults to https://trialeservices.yeka.gr/WebServicesAPI/api.

```rust
let ergani_username = env::var("ERGANI_USERNAME") ?;
let ergani_password = env::var("ERGANI_PASSWORD") ?;
let ergani_base_url = env::var("ERGANI_BASE_URL")
.unwrap_or("https://trialeservices.yeka.gr/WebServicesAPI/api".to_string());

let client = ErganiClient::new(ergani_username, ergani_password, Some(ergani_base_url)).await?;
```

If you intend to use this package for multiple company entities, it is necessary to create separate client instances for
each entity with the appropriate credentials.

We are going to use the aforementioned `ergani_client` for the following usage examples.

### Work card

Submit work card records to Ergani in order to declare an employee's movement (arrival, departure).

```rust
async fn submit_work_card(company_work_cards: Vec<CompanyWorkCard>) -> Result<Vec<SubmissionResponse>>
```

#### Example

```rust
let work_card = vec![
    CompanyWorkCardBuilder::builder()
        .set_employer_tax_identification_number("0123456789".to_string())
        .set_business_branch_number(12)
        .set_comments(Some("Σχόλia".to_string()))
        .set_card_details(vec![
            WorkCardBuilder::builder()
                .set_employee_tax_identification_number("0123456789".to_string())
                .set_employee_last_name("Last".to_string())
                .set_employee_first_name("First".to_string())
                .set_work_card_movement_type(WorkCardMovementType::Arrival)
                .set_work_card_submission_date(NaiveDate::parse_from_str("2021-01-01", "%Y-%m-%d").unwrap())
                .set_work_card_movement_datetime(DateTime::parse_from_str("2021-01-01T12:00", "%Y-%m-%dT%H:%M").unwrap().to_utc())
                .set_late_declaration_justification(Some(LateDeclarationJustificationType::PowerOutage))
                .build()?
        ])
        .build()
];

ergani_client.submit_work_card(work_card).await?;
```

**Note:** You can submit work cards for various employees across multiple company branches simultaneously as shown
above.

### Overtime

Submit overtime records to Ergani in order to declare employees overtimes.

```rust
async fn submit_overtime(company_overtimes: Vec<CompanyOvertime>) -> Result<Vec<SubmissionResponse>>
```

#### Example

```rust
let company_overtimes = vec![
    CompanyOvertimeBuilder::builder()
        .set_business_branch_number(12)
        .set_sepe_service_code("10".to_string())
        .set_business_primary_activity_code("100".to_string())
        .set_business_branch_activity_code("101".to_string())
        .set_kallikratis_municipal_code("100".to_string())
        .set_legal_representative_tax_identification_number("0123456789".to_string())
        .set_employee_overtimes(vec![OvertimeBuilder::builder()
            .set_employee_tax_identification_number("0123456789".to_string())
            .set_employee_social_security_number("0123456789".to_string())
            .set_employee_last_name("Last".to_string())
            .set_employee_first_name("First".to_string())
            .set_overtime_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
            .set_overtime_start_time("2024-03-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
            .set_overtime_end_time("2024-03-01T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
            .set_overtime_cancellation(false)
            .set_employee_profession_code("".to_string())
            .set_overtime_justification(
                OvertimeJustificationType::AccidentPreventionOrDamageRestoration,
            )
            .set_weekly_workdays_number(WeeklyWorkDays::Five)
            .set_asee_approval(Some("ΑΣΕΕ".to_string()))
            .build()
            .unwrap()])
        .set_related_protocol_id(Some("Αρ. Πρωτ. Σχετ.".to_string()))
        .set_related_protocol_date(Some(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()))
        .set_employer_organization(Some("Εργοδότης".to_string()))
        .set_business_secondary_activity_code_1(Some("ΚΑΔ 1".to_string()))
        .set_business_secondary_activity_code_2(Some("ΚΑΔ 2".to_string()))
        .set_business_secondary_activity_code_3(Some("ΚΑΔ 3".to_string()))
        .set_business_secondary_activity_code_4(Some("ΚΑΔ 4".to_string()))
        .set_comments(Some("Σχόλια".to_string()))
        .build(),
];

ergani_client.submit_overtime(company_overtimes).await?;
```

**Note:** You can submit overtime records for various employees across multiple company branches simultaneously.

### Daily schedule

Submit daily schedules to Ergani in order to declare schedules for employees that don't have a fixed schedule (e.g.
shift workers).

```rust
async fn submit_daily_schedule(company_daily_schedules: Vec<CompanyDailySchedule>) -> Result<Vec<SubmissionResponse>>
```

#### Example

```rust
let company_daily_schedules = vec![
    CompanyDailyScheduleBuilder::builder()
        .set_business_branch_number(12)
        .set_start_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
        .set_end_date(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap())
        .set_employee_schedules(vec![
            EmployeeDailyScheduleBuilder::builder()
                .set_employee_tax_identification_number("0123456789".to_string())
                .set_employee_last_name("Last".to_string())
                .set_employee_first_name("First".to_string())
                .set_schedule_date(NaiveDate::from_ymd_opt(2024, 3, 3).unwrap())
                .set_workday_details(vec![
                    WorkDayDetailsBuilder::builder()
                        .set_work_type(ScheduleWorkType::WorkFromHome)
                        .set_start_time("2024-03-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .set_end_time("2024-03-01T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .build()?,
                    WorkDayDetailsBuilder::builder()
                        .set_work_type(ScheduleWorkType::WorkFromOffice)
                        .set_start_time("2024-03-02T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .set_end_time("2024-03-02T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .build()?,
                ])
                .build()
        ])
        .set_related_protocol_id(Some("1".to_string()))
        .set_related_protocol_date(Some(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()))
        .set_comments(Some("Σχόλια".to_string()))
        .build()
];

ergani_client
.submit_daily_schedule(company_daily_schedules)
.await?;
```

### Weekly schedule

Submit weekly schedules to Ergani in order to declare schedules for employees that have a fixed schedule.

```rust
async fn submit_weekly_schedule(company_weekly_schedules: Vec<CompanyWeeklySchedule>) -> Result<Vec<SubmissionResponse>>
```

#### Example

```rust
let company_weekly_schedules = vec![
    CompanyWeeklyScheduleBuilder::builder()
        .set_business_branch_number(10)
        .set_start_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
        .set_end_date(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap())
        .set_employee_schedules(vec![
            EmployeeWeeklyScheduleBuilder::builder()
                .set_employee_tax_identification_number("0123456789".to_string())
                .set_employee_last_name("Last".to_string())
                .set_employee_first_name("First".to_string())
                .set_schedule_date(NaiveDate::from_ymd_opt(2024, 3, 3).unwrap())
                .set_workday_details(vec![
                    WorkDayDetailsBuilder::builder()
                        .set_work_type(ScheduleWorkType::WorkFromHome)
                        .set_start_time("2024-03-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .set_end_time("2024-03-01T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .build()?,
                    WorkDayDetailsBuilder::builder()
                        .set_work_type(ScheduleWorkType::WorkFromOffice)
                        .set_start_time("2024-03-02T12:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .set_end_time("2024-03-02T20:00:00Z".parse::<DateTime<Utc>>().unwrap())
                        .build()?,
                ])
                .build()
        ])
        .set_related_protocol_id(Some("1".to_string()))
        .set_related_protocol_date(Some(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()))
        .build()
];

ergani_client
.submit_weekly_schedule(company_weekly_schedules)
.await?;
```

**Note:** You can submit weekly schedules for various employees across multiple company branches simultaneously.

---

Full reference documentation is available at [https://ergani.withlogic.dev/](https://ergani.withlogic.dev/).

## Glossary

The glossary might help you if you're taking a look at the official documentation of the Ergani
API (https://eservices.yeka.gr/(S(ayldvlj35eukgvmzrr055oe5))/Announcements.aspx?id=257).

### Work card

| **Original**       | **Original help text** (in Greek)              | **Translated**                        |
|--------------------|------------------------------------------------|---------------------------------------|
| `f_afm_ergodoti`   | Α.Φ.Μ Εργοδότη (Για επαλήθευση)                | `employer_tax_identification_number`  |
| `f_aa`             | Α/Α Παραρτήματος                               | `business_branch_number`              |
| `f_comments`       | ΣΧΟΛΙΑ                                         | `comments`                            |
| `f_afm`            | ΑΡΙΘΜΟΣ ΦΟΡΟΛΟΓΙΚΟΥ ΜΗΤΡΩΟΥ (Α.Φ.Μ.)           | `employee_tax_indentification_number` |
| `f_eponymo`        | ΕΠΩΝΥΜΟ                                        | `employee_last_name`                  |
| `f_onoma`          | ΟΝΟΜΑ                                          | `employee_first_name`                 |
| `f_type`           | Τύπος Κίνησης                                  | `work_card_movement_type`             |
| `f_reference_date` | ΗΜ/ΝΙΑ Αναφοράς                                | `work_card_submission_date`           |
| `f_date`           | ΗΜ/ΝΙΑ Κίνησης                                 | `work_card_movement_datetime`         |
| `f_aitiologia`     | ΚΩΔΙΚΟΣ ΑΙΤΙΟΛΟΓΙΑΣ (Σε περίπτωση Εκπρόθεσμου) | `late_declaration_justification`      |

#### Work card movement types

| **Original API code** | **Original help text** (in Greek) | **Translated** |
|-----------------------|-----------------------------------|----------------|
| `0`                   | ΠΡΟΣΕΛΕΥΣΗ                        | `ARRIVAL`      |
| `1`                   | ΑΠΟΧΩΡΗΣΗ                         | `DEPARTURE`    |

#### Work card justifications

| **Original API code** | **Original help text** (in Greek)           | **Translated**                 |
|-----------------------|---------------------------------------------|--------------------------------|
| `001`                 | ΠΡΟΒΛΗΜΑ ΣΤΗΝ ΗΛΕΚΤΡΟΔΟΤΗΣΗ/ΤΗΛΕΠΙΚΟΙΝΩΝΙΕΣ | `POWER_OUTAGE`                 |
| `002`                 | ΠΡΟΒΛΗΜΑ ΣΤΑ ΣΥΣΤΗΜΑΤΑ ΤΟΥ ΕΡΓΟΔΟΤΗ         | `EMPLOYER_SYSTEMS_UNAVAILABLE` |
| `003`                 | ΠΡΟΒΛΗΜΑ ΣΥΝΔΕΣΗΣ ΜΕ ΤΟ ΠΣ ΕΡΓΑΝΗ           | `ERGANI_SYSTEMS_UNAVAILABLE`   |

### Overtime

| **Original**                 | **Original help text** (in Greek)                 | **Translated**                                   |
|------------------------------|---------------------------------------------------|--------------------------------------------------|
| `f_aa`                       | Α/Α Παραρτήματος                                  | `business_branch_number`                         |
| `f_rel_protocol`             | ΣΧΕΤΙΚΟ ΕΝΤΥΠΟ ΑΡΙΘ. ΠΡΩΤ.	                       | `related_protocol_id`                            |
| `f_rel_date`                 | ΣΧΕΤΙΚΟ ΕΝΤΥΠΟ ΗΜΕΡΟΜΗΝΙΑ	                        | `related_protocol_date`                          |
| `f_ypiresia_sepe`            | ΚΩΔΙΚΟΣ ΥΠΗΡΕΣΙΑΣ ΣΕΠΕ	                           | `sepe_service_code`                              |
| `f_ergodotikh_organwsh`      | ΕΡΓΟΔΟΤΙΚΗ ΟΡΓΑΝΩΣΗ	                              | `employer_organization`                          |
| `f_kad_kyria`                | Κ.Α.Δ. - ΚΥΡΙΑ ΔΡΑΣΤΗΡΙΟΤΗΤΑ	                     | `business_primary_activity_code`                 |
| `f_kad_deyt_1`               | Κ.Α.Δ. - ΚΥΡΙΑ ΔΡΑΣΤΗΡΙΟΤΗΤΑ	1                    | `business_secondary_activity_code_1`             |
| `f_kad_deyt_2`               | Κ.Α.Δ. - ΚΥΡΙΑ ΔΡΑΣΤΗΡΙΟΤΗΤΑ	2                    | `business_secondary_activity_code_2`             |
| `f_kad_deyt_3`               | Κ.Α.Δ. - ΚΥΡΙΑ ΔΡΑΣΤΗΡΙΟΤΗΤΑ	3                    | `business_secondary_activity_code_3`             |
| `f_kad_deyt_4`               | Κ.Α.Δ. - ΚΥΡΙΑ ΔΡΑΣΤΗΡΙΟΤΗΤΑ	4                    | `business_secondary_activity_code_4`             |
| `f_kad_pararthmatos`         | Κ.Α.Δ. ΠΑΡΑΡΤΗΜΑΤΟΣ	                              | `business_brach_activity_code`                   |
| `f_kallikratis_pararthmatos` | ΔΗΜΟΤΙΚΗ / ΤΟΠΙΚΗ ΚΟΙΝΟΤΗΤΑ	                      | `kallikratis_municipal_code`                     |
| `f_comments`                 | ΠΑΡΑΤΗΡΗΣΕΙΣ                                      | `comments`                                       |
| `f_afm_proswpoy`             | Νόμιμος Εκπρόσωπος(Α.Φ.Μ.)                        | `legal_representative_tax_identification_number` |
| `f_afm`                      | ΑΡΙΘΜΟΣ ΦΟΡΟΛΟΓΙΚΟΥ ΜΗΤΡΩΟΥ (Α.Φ.Μ.)              | `employee_tax_indentification_number`            |
| `f_amka`                     | ΑΡΙΘΜΟΣ ΜΗΤΡΩΟΥ ΚΟΙΝΩΝΙΚΗΣ ΑΣΦΑΛΙΣΗΣ (Α.Μ.Κ.Α.)   | `employee_social_security_number`                |
| `f_eponymo`                  | ΕΠΩΝΥΜΟ                                           | `employee_last_name`                             |
| `f_onoma`                    | ΟΝΟΜΑ                                             | `employee_first_name`                            |
| `f_date`                     | ΗΜΕΡΟΜΗΝΙΑ ΥΠΕΡΩΡΙΑΣ	                             | `overtime_date`                                  |
| `f_from`                     | ΩΡΑ ΕΝΑΡΞΗΣ ΥΠΕΡΩΡΙΑΣ (HH24:MM)	                  | `overtime_start_time`                            |
| `f_to`                       | ΩΡΑ ΛΗΞΗΣ ΥΠΕΡΩΡΙΑΣ (HH24:MM)	                    | `overtime_end_time`                              |
| `f_cancellation`             | ΑΚΥΡΩΣΗ ΥΠΕΡΩΡΙΑΣ	                                | `overtime_cancellation`                          |
| `f_step`                     | ΕΙΔΙΚΟΤΗΤΑ ΚΩΔΙΚΟΣ	                               | `employee_profession_code`                       |
| `f_reason`                   | ΑΙΤΙΟΛΟΓΙΑ ΚΩΔΙΚΟΣ	                               | `overtime_justification`                         |
| `f_weekdates`                | ΕΒΔΟΜΑΔΙΑΙΑ ΑΠΑΣΧΟΛΗΣΗ (5) ΠΕΝΘΗΜΕΡΟ (6) ΕΞΑΗΜΕΡΟ | `weekly_workdays_number`                         |
| `f_asee`                     | ΕΓΚΡΙΣΗ ΑΣΕΕ	                                     | `asee_approval`                                  |

#### Overtime justfications

| **Original API code** | **Original help text** (in Greek)                                                 | Translation                                 |
|-----------------------|-----------------------------------------------------------------------------------|---------------------------------------------|
| `001`                 | ΠΡΟΛΗΨΗ ΑΤΥΧΗΜΑΤΩΝ Η ΑΠΟΚΑΤΑΣΤΑΣΗ ΖΗΜΙΩΝ                                          | `ACCIDENT_PREVENTION_OR_DAMAGE_RESTORATION` |
| `002`                 | ΕΠΕΙΓΟΥΣΕΣ ΕΡΓΑΣΙΕΣ ΕΠΟΧΙΑΚΟΥ ΧΑΡΑΚΤΗΡΑ                                           | `URGENT_SEASONAL_TASKS`                     |
| `003`                 | ΕΞΑΙΡΕΤΙΚΗ ΣΩΡΕΥΣΗ ΕΡΓΑΣΙΑΣ – ΦΟΡΤΟΣ ΕΡΓΑΣΙΑΣ                                     | `EXCEPTIONAL_WORKLOAD`                      |
| `004`                 | ΠΡΟΕΠΙΣΚΕΥΑΣΤΙΚΕΣ Η ΣΥΜΠΛΗΡΩΜΑΤΙΚΕΣ ΕΡΓΑΣΙΕΣ                                      | `SUPPLEMENTARY_TASKS`                       |
| `005`                 | ΑΝΑΠΛΗΡΩΣΗ ΧΑΜΕΝΩΝ ΩΡΩΝ ΛΟΓΩ ΞΑΦΝΙΚΩΝ ΑΙΤΙΩΝ Η ΑΝΩΤΕΡΑΣ ΒΙΑΣ                      | `LOST_HOURS_SUDDEN_CAUSES`                  |
| `006`                 | ΑΝΑΠΛΗΡΩΣΗ ΧΑΜΕΝΩΝ ΩΡΩΝ ΛΟΓΩ ΕΠΙΣΗΜΩΝ ΑΡΓΙΩΝ                                      | `LOST_HOURS_OFFICIAL_HOLIDAYS`              |
| `007`                 | ΑΝΑΠΛΗΡΩΣΗ ΧΑΜΕΝΩΝ ΩΡΩΝ ΛΟΓΩ ΚΑΙΡΙΚΩΝ ΣΥΝΘΗΚΩΝ                                    | `LOST_HOURS_WEATHER_CONDITIONS`             |
| `008`                 | ΈΚΤΑΚΤΕΣ ΕΡΓΑΣΙΕΣ ΚΛΕΙΣΙΜΑΤΟΣ ΗΜΕΡΑΣ Η ΜΗΝΑ                                       | `EMERGENCY_CLOSURE_DAY`                     |
| `009`                 | ΛΟΙΠΕΣ ΕΡΓΑΣΙΕΣ ΟΙ ΟΠΟΙΕΣ ΔΕΝ ΜΠΟΡΟΥΝ ΝΑ ΠΡΑΓΜΑΤΟΠΟΙΗΘΟΥΝ ΚΑΤΑ ΤΙΣ ΕΡΓΑΣΙΜΕΣ ΩΡΕΣ | `NON_WORKDAY_TASKS`                         |

### Daily schedule

| **Original**        | **Original help text** (in Greek)    | **Translated**                        |
|---------------------|--------------------------------------|---------------------------------------|
| `f_aa_pararthmatos` | Α/Α ΠΑΡΑΡΤΗΜΑΤΟΣ                     | `business_branch_number`              |
| `f_rel_protocol`    | ΣΧΕΤΙΚΟ ΕΝΤΥΠΟ ΑΡΙΘ. ΠΡΩΤ.           | `related_protocol_id`                 |
| `f_rel_date`        | ΣΧΕΤΙΚΟ ΕΝΤΥΠΟ ΗΜΕΡΟΜΗΝΙΑ            | `related_protocol_date`               |
| `f_comments`        | ΠΑΡΑΤΗΡΗΣΕΙΣ                         | `comments`                            |
| `f_from_date`       | ΗΜΕΡΟΜΗΝΙΑ ΑΠΟ                       | `start_date`                          |
| `f_to_date`         | ΗΜΕΡΟΜΗΝΙΑ ΕΩΣ                       | `end_date`                            |
| `f_afm`             | ΑΡΙΘΜΟΣ ΦΟΡΟΛΟΓΙΚΟΥ ΜΗΤΡΩΟΥ (Α.Φ.Μ.) | `employee_tax_indentification_number` |
| `f_eponymo`         | ΕΠΩΝΥΜΟ                              | `employee_last_name`                  |
| `f_onoma`           | ΟΝΟΜΑ                                | `employee_first_name`                 |
| `f_day`             | ΗΜΕΡΑ                                | `schedule_date`                       |
| `f_type`            | ΤΥΠΟΣ ΑΝΑΛΥΤΙΚΗΣ ΕΓΓΡΑΦΗΣ - ΚΩΔΙΚΟΣ  | `work_type`                           |
| `f_from`            | ΩΡΑ ΑΠΟ (HH24:MM)                    | `start_time`                          |
| `f_to`              | ΩΡΑ ΕΩΣ (HH24:MM)                    | `end_time`                            |

### Weekly schedule

| **Original**        | **Original help text** (in Greek)    | **Translated**                        |
|---------------------|--------------------------------------|---------------------------------------|
| `f_aa_pararthmatos` | Α/Α ΠΑΡΑΡΤΗΜΑΤΟΣ                     | `business_brach_number`               |
| `f_rel_protocol`    | ΣΧΕΤΙΚΟ ΕΝΤΥΠΟ ΑΡΙΘ. ΠΡΩΤ.           | `related_protocol_id`                 |
| `f_rel_date`        | ΣΧΕΤΙΚΟ ΕΝΤΥΠΟ ΗΜΕΡΟΜΗΝΙΑ            | `related_protocol_date`               |
| `f_comments`        | ΠΑΡΑΤΗΡΗΣΕΙΣ                         | `comments`                            |
| `f_from_date`       | ΗΜΕΡΟΜΗΝΙΑ ΑΠΟ                       | `start_date`                          |
| `f_to_date`         | ΗΜΕΡΟΜΗΝΙΑ ΕΩΣ                       | `end_date`                            |
| `f_afm`             | ΑΡΙΘΜΟΣ ΦΟΡΟΛΟΓΙΚΟΥ ΜΗΤΡΩΟΥ (Α.Φ.Μ.) | `employee_tax_indentification_number` |
| `f_eponymo`         | ΕΠΩΝΥΜΟ                              | `employee_last_name`                  |
| `f_onoma`           | ΟΝΟΜΑ                                | `employee_first_name`                 |
| `f_date`            | ΗΜΕΡΟΜΗΝΙΑ                           | `schedule_date`                       |
| `f_type`            | ΤΥΠΟΣ ΑΝΑΛΥΤΙΚΗΣ ΕΓΓΡΑΦΗΣ - ΚΩΔΙΚΟΣ  | `work_type`                           |
| `f_from`            | ΩΡΑ ΑΠΟ (HH24:MM)                    | `start_time`                          |
| `f_to`              | ΩΡΑ ΕΩΣ (HH24:MM)                    | `end_time`                            |

### Schedule work types

| **Original API code** | **Original help text** (in Greek) | **Translated**     |
|-----------------------|-----------------------------------|--------------------|
| `ΜΕ`                  | ΜΗ ΕΡΓΑΣΙΑ                        | `ABSENT`           |
| `ΑΝ`                  | ΑΝΑΠΑΥΣΗ/ΡΕΠΟ                     | `REST_DAY`         |
| `ΤΗΛ`                 | ΤΗΛΕΡΓΑΣΙΑ                        | `WORK_FROM_HOME`   |
| `ΕΡΓ`                 | ΕΡΓΑΣΙΑ                           | `WORK_FROM_OFFICE` |

## License

This project is licensed under the [`MIT License`](LICENSE)
