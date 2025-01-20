use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::auth::authenticator::ErganiAuthenticationState;
use ergani::client::ErganiClient;
use anyhow::Result;

#[allow(dead_code)]
pub(crate) async fn fetch_overtimes(
    ergani_client: &ErganiClient,
    auth_state: ErganiAuthenticationState,
) -> Result<()> {
    let overtimes = ergani_client.fetch_overtimes(auth_state).await?;

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
            Cell::new(overtime.f_aa_pararthmatos.to_string()),
            Cell::new(overtime.f_rel_protocol.to_string()),
            Cell::new(overtime.f_rel_date.to_string()),
            Cell::new(overtime.f_ypiresia_sepe.to_string()),
            Cell::new(overtime.f_ergodotikh_organwsh.to_string()),
            Cell::new(overtime.f_kad_kyria.to_string()),
            Cell::new(overtime.f_kad_deyt_1.to_string()),
            Cell::new(overtime.f_kad_deyt_2.to_string()),
            Cell::new(overtime.f_kad_deyt_3.to_string()),
            Cell::new(overtime.f_kad_deyt_4.to_string()),
            Cell::new(overtime.f_kad_pararthmatos.to_string()),
            Cell::new(overtime.f_kallikratis_pararthmatos.to_string()),
            Cell::new(overtime.f_comments.to_string()),
            Cell::new(overtime.f_afm_proswpoy.to_string()),
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
                Cell::new(overtime_ergazomenos.f_afm.to_string()),
                Cell::new(overtime_ergazomenos.f_amka.to_string()),
                Cell::new(overtime_ergazomenos.f_eponymo.to_string()),
                Cell::new(overtime_ergazomenos.f_onoma.to_string()),
                Cell::new(overtime_ergazomenos.f_date.to_string()),
                Cell::new(overtime_ergazomenos.f_from.to_string()),
                Cell::new(overtime_ergazomenos.f_to.to_string()),
                Cell::new(overtime_ergazomenos.f_from_2.to_string()),
                Cell::new(overtime_ergazomenos.f_to_2.to_string()),
                Cell::new(overtime_ergazomenos.f_cancellation.to_string()),
                Cell::new(overtime_ergazomenos.f_step.to_string()),
                Cell::new(overtime_ergazomenos.f_reason.to_string()),
                Cell::new(overtime_ergazomenos.f_weekdates.to_string()),
                Cell::new(overtime_ergazomenos.f_asee.to_string()),
            ]);
        }

        overtime_tables.push(overtime_ergazomenos_table);
    }

    for table in overtime_tables {
        println!("{table}");
    }

    Ok(())
}
