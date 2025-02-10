use anyhow::Result;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::auth::authenticator::ErganiAuthenticationState;
use ergani::client::ErganiClient;

#[allow(dead_code)]
pub(crate) async fn fetch_weekly_schedule(
    ergani_client: &ErganiClient,
    auth_state: ErganiAuthenticationState,
) -> Result<()> {
    let week_schedule = ergani_client.fetch_weekly_schedule(auth_state).await?;

    let mut weekly_schedule_tables: Vec<Table> = vec![];

    for wto in week_schedule.response().unwrap().week_schedule.wtos.wto.iter() {
        let mut wto_table = Table::new();
        wto_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Parartima").add_attribute(Attribute::Bold),
                Cell::new("Comments").add_attribute(Attribute::Bold),
                Cell::new("From").add_attribute(Attribute::Bold),
                Cell::new("To").add_attribute(Attribute::Bold),
            ]);

        wto_table.add_row(vec![
            Cell::new(wto.f_aa_pararthmatos.to_string()),
            Cell::new(wto.f_comments.to_string()),
            Cell::new(wto.f_from_date.to_string()),
            Cell::new(wto.f_to_date.to_string()),
        ]);

        weekly_schedule_tables.push(wto_table);

        for ergazomenos in wto.ergazomenoi.ergazomenoi_wto.iter() {
            let mut ergazomenos_table = Table::new();
            ergazomenos_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("AFM").add_attribute(Attribute::Bold),
                    Cell::new("Eponymo").add_attribute(Attribute::Bold),
                    Cell::new("Onoma").add_attribute(Attribute::Bold),
                ]);

            ergazomenos_table.add_row(vec![
                Cell::new(ergazomenos.f_afm.to_string()),
                Cell::new(ergazomenos.f_eponymo.to_string()),
                Cell::new(ergazomenos.f_onoma.to_string()),
            ]);

            weekly_schedule_tables.push(ergazomenos_table);

            let mut ergazomenos_analytic_table = Table::new();
            ergazomenos_analytic_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("Type").add_attribute(Attribute::Bold),
                    Cell::new("From").add_attribute(Attribute::Bold),
                    Cell::new("To").add_attribute(Attribute::Bold),
                ]);

            for ergazomenos_analytic in ergazomenos.ergazomenos_analytics.ergazomenos_wtoanalytics.iter() {
                ergazomenos_analytic_table.add_row(vec![
                    Cell::new(ergazomenos_analytic.f_type.to_string()),
                    Cell::new(ergazomenos_analytic.f_from.to_string()),
                    Cell::new(ergazomenos_analytic.f_to.to_string()),
                ]);
            }

            weekly_schedule_tables.push(ergazomenos_analytic_table);
        }
    }

    for table in weekly_schedule_tables {
        println!("{table}");
    }

    Ok(())
}
