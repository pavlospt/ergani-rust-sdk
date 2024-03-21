use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::client::ErganiClient;

#[allow(dead_code)]
pub(crate) async fn fetch_daily_schedule(ergani_client: &ErganiClient) -> anyhow::Result<()> {
    let day_schedule = ergani_client.fetch_daily_schedule().await?;

    let mut daily_schedule_tables: Vec<Table> = vec![];

    for (index, wto) in day_schedule.day_schedules.wtos.wto.into_iter().enumerate() {
        let mut wto_table = Table::new();
        wto_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("Parartima").add_attribute(Attribute::Bold),
                Cell::new("Rel Protocol").add_attribute(Attribute::Bold),
                Cell::new("Rel Date").add_attribute(Attribute::Bold),
                Cell::new("Comments").add_attribute(Attribute::Bold),
                Cell::new("From Date").add_attribute(Attribute::Bold),
                Cell::new("To Date").add_attribute(Attribute::Bold),
            ]);

        let current_index = index + 1;

        wto_table.add_row(vec![
            Cell::new(format!("{}:{current_index}", wto.f_aa_pararthmatos)),
            Cell::new(format!("{}:{current_index}", wto.f_rel_protocol)),
            Cell::new(format!("{}:{current_index}", wto.f_rel_date)),
            Cell::new(format!("{}:{current_index}", wto.f_comments)),
            Cell::new(format!("{}:{current_index}", wto.f_from_date)),
            Cell::new(format!("{}:{current_index}", wto.f_to_date)),
        ]);

        daily_schedule_tables.push(wto_table);

        for (ergazomenos_index, ergazomenos) in
            wto.ergazomenoi.ergazomenoi_wto.into_iter().enumerate()
        {
            let mut ergazomenos_table = Table::new();
            ergazomenos_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(80)
                .set_header(vec![
                    Cell::new("AFM").add_attribute(Attribute::Bold),
                    Cell::new("Eponymo").add_attribute(Attribute::Bold),
                    Cell::new("Onoma").add_attribute(Attribute::Bold),
                ]);

            let current_ergazomenos_index = ergazomenos_index + 1;

            ergazomenos_table.add_row(vec![
                Cell::new(format!("{}:{current_ergazomenos_index}", ergazomenos.f_afm)),
                Cell::new(format!(
                    "{}:{current_ergazomenos_index}",
                    ergazomenos.f_eponymo
                )),
                Cell::new(format!(
                    "{}:{current_ergazomenos_index}",
                    ergazomenos.f_onoma
                )),
            ]);

            daily_schedule_tables.push(ergazomenos_table);

            for (ergazomenos_analytics_index, ergazomenos_analytic) in ergazomenos
                .ergazomenos_analytics
                .ergazomenos_wtoanalytics
                .into_iter()
                .enumerate()
            {
                let mut ergazomenos_analytic_table = Table::new();
                ergazomenos_analytic_table
                    .load_preset(UTF8_FULL)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_width(80)
                    .set_header(vec![
                        Cell::new("Type").add_attribute(Attribute::Bold),
                        Cell::new("From").add_attribute(Attribute::Bold),
                        Cell::new("To").add_attribute(Attribute::Bold),
                    ]);

                let current_ergazomenos_analytic_index = ergazomenos_analytics_index + 1;

                ergazomenos_analytic_table.add_row(vec![
                    Cell::new(format!(
                        "{}:{current_ergazomenos_analytic_index}",
                        ergazomenos_analytic.f_type
                    )),
                    Cell::new(format!(
                        "{}:{current_ergazomenos_analytic_index}",
                        ergazomenos_analytic.f_from
                    )),
                    Cell::new(format!(
                        "{}:{current_ergazomenos_analytic_index}",
                        ergazomenos_analytic.f_to
                    )),
                ]);

                daily_schedule_tables.push(ergazomenos_analytic_table);
            }
        }
    }

    for table in daily_schedule_tables {
        println!("{table}");
    }

    Ok(())
}
