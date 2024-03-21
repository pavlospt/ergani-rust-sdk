use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::client::ErganiClient;

#[allow(dead_code)]
pub(crate) async fn fetch_overtimes(ergani_client: &ErganiClient) -> anyhow::Result<()> {
    let overtimes = ergani_client.fetch_overtimes().await?;

    let mut overtime_tables: Vec<Table> = vec![];

    for overtime in overtimes.overtimes.overtimes.overtime {
        let mut overtime_table = Table::new();
        overtime_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("Parartima").add_attribute(Attribute::Bold),
                Cell::new("Rel Protocol").add_attribute(Attribute::Bold),
                Cell::new("Rel Date").add_attribute(Attribute::Bold),
                Cell::new("Ypiresia SEPE").add_attribute(Attribute::Bold),
                Cell::new("Ergodotikh Organwsh").add_attribute(Attribute::Bold),
                Cell::new("KAD Kyria").add_attribute(Attribute::Bold),
                Cell::new("KAD Deyt1").add_attribute(Attribute::Bold),
                Cell::new("KAD Deyt2").add_attribute(Attribute::Bold),
                Cell::new("KAD Deyt3").add_attribute(Attribute::Bold),
                Cell::new("KAD Deyt4").add_attribute(Attribute::Bold),
                Cell::new("KAD Parartimatos").add_attribute(Attribute::Bold),
                Cell::new("Kallikratis Parartimatos").add_attribute(Attribute::Bold),
                Cell::new("Comments").add_attribute(Attribute::Bold),
                Cell::new("AFM Proswpoy").add_attribute(Attribute::Bold),
            ]);

        overtime_table.add_row(vec![
            Cell::new(format!("{}", overtime.f_aa_pararthmatos)),
            Cell::new(format!("{}", overtime.f_rel_protocol)),
            Cell::new(format!("{}", overtime.f_rel_date)),
            Cell::new(format!("{}", overtime.f_ypiresia_sepe)),
            Cell::new(format!("{}", overtime.f_ergodotikh_organwsh)),
            Cell::new(format!("{}", overtime.f_kad_kyria)),
            Cell::new(format!("{}", overtime.f_kad_deyt_1)),
            Cell::new(format!("{}", overtime.f_kad_deyt_2)),
            Cell::new(format!("{}", overtime.f_kad_deyt_3)),
            Cell::new(format!("{}", overtime.f_kad_deyt_4)),
            Cell::new(format!("{}", overtime.f_kad_pararthmatos)),
            Cell::new(format!("{}", overtime.f_kallikratis_pararthmatos)),
            Cell::new(format!("{}", overtime.f_comments)),
            Cell::new(format!("{}", overtime.f_afm_proswpoy)),
        ]);

        overtime_tables.push(overtime_table);

        let mut overtime_ergazomenos_table = Table::new();
        overtime_ergazomenos_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("AFM").add_attribute(Attribute::Bold),
                Cell::new("AMKA").add_attribute(Attribute::Bold),
                Cell::new("Eponymo").add_attribute(Attribute::Bold),
                Cell::new("Onoma").add_attribute(Attribute::Bold),
                Cell::new("Date").add_attribute(Attribute::Bold),
                Cell::new("From").add_attribute(Attribute::Bold),
                Cell::new("To").add_attribute(Attribute::Bold),
                Cell::new("From 2").add_attribute(Attribute::Bold),
                Cell::new("To 2").add_attribute(Attribute::Bold),
                Cell::new("Cancellation").add_attribute(Attribute::Bold),
                Cell::new("Step").add_attribute(Attribute::Bold),
                Cell::new("Reason").add_attribute(Attribute::Bold),
                Cell::new("Weekdates").add_attribute(Attribute::Bold),
                Cell::new("ASEE").add_attribute(Attribute::Bold),
            ]);

        for overtime_ergazomenos in overtime.ergazomenoi.overtime_ergazomenos_date {
            overtime_ergazomenos_table.add_row(vec![
                Cell::new(format!("{}", overtime_ergazomenos.f_afm)),
                Cell::new(format!("{}", overtime_ergazomenos.f_amka)),
                Cell::new(format!("{}", overtime_ergazomenos.f_eponymo)),
                Cell::new(format!("{}", overtime_ergazomenos.f_onoma)),
                Cell::new(format!("{}", overtime_ergazomenos.f_date)),
                Cell::new(format!("{}", overtime_ergazomenos.f_from)),
                Cell::new(format!("{}", overtime_ergazomenos.f_to)),
                Cell::new(format!("{}", overtime_ergazomenos.f_from_2)),
                Cell::new(format!("{}", overtime_ergazomenos.f_to_2)),
                Cell::new(format!("{}", overtime_ergazomenos.f_cancellation)),
                Cell::new(format!("{}", overtime_ergazomenos.f_step)),
                Cell::new(format!("{}", overtime_ergazomenos.f_reason)),
                Cell::new(format!("{}", overtime_ergazomenos.f_weekdates)),
                Cell::new(format!("{}", overtime_ergazomenos.f_asee)),
            ]);
        }

        overtime_tables.push(overtime_ergazomenos_table);
    }

    for table in overtime_tables {
        println!("{table}");
    }

    Ok(())
}
