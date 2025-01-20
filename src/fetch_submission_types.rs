use anyhow::Result;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::auth::authenticator::ErganiAuthenticationState;
use ergani::client::ErganiClient;

#[allow(dead_code)]
pub(crate) async fn fetch_submission_types(
    ergani_client: &ErganiClient,
    auth_state: ErganiAuthenticationState,
) -> Result<()> {
    let submission_types = ergani_client.fetch_submissions(auth_state).await?;

    let mut submission_type_tables: Vec<Table> = vec![];
    let mut submission_type_table = Table::new();

    submission_type_table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Submission Type ID").add_attribute(Attribute::Bold),
            Cell::new("Submission Code").add_attribute(Attribute::Bold),
            Cell::new("Submission Description").add_attribute(Attribute::Bold),
        ]);

    for submission_type in submission_types {
        submission_type_table.add_row(vec![
            Cell::new(format!("{}", submission_type.id)),
            Cell::new(submission_type.code.to_string()),
            Cell::new(submission_type.description.to_string()),
        ]);
    }

    submission_type_tables.push(submission_type_table);

    for table in submission_type_tables {
        println!("{table}");
    }

    Ok(())
}
