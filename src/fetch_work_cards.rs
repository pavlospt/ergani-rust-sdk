use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::client::ErganiClient;

#[allow(dead_code)]
pub(crate) async fn fetch_work_cards(ergani_client: &ErganiClient) -> anyhow::Result<()> {
    let work_cards = ergani_client.fetch_work_cards().await?;

    let mut work_cards_tables: Vec<Table> = vec![];

    for (index, work_card) in work_cards.work_cards.cards.card.into_iter().enumerate() {
        let mut work_card_table = Table::new();
        work_card_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("AFM Ergodoti").add_attribute(Attribute::Bold),
                Cell::new("AA").add_attribute(Attribute::Bold),
                Cell::new("Comments").add_attribute(Attribute::Bold),
            ]);

        let current_index = index + 1;

        work_card_table.add_row(vec![
            Cell::new(format!("{}:{current_index}", work_card.f_afm_ergodoti)),
            Cell::new(format!("{}:{current_index}", work_card.f_aa)),
            Cell::new(format!("{}:{current_index}", work_card.f_comments)),
        ]);

        work_cards_tables.push(work_card_table);

        for (card_index, card_detail) in work_card.details.card_details.into_iter().enumerate() {
            let mut card_detail_table = Table::new();
            card_detail_table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(80)
                .set_header(vec![
                    Cell::new("AFM").add_attribute(Attribute::Bold),
                    Cell::new("Eponymo").add_attribute(Attribute::Bold),
                    Cell::new("Onoma").add_attribute(Attribute::Bold),
                    Cell::new("Type").add_attribute(Attribute::Bold),
                    Cell::new("Reference Date").add_attribute(Attribute::Bold),
                    Cell::new("Date").add_attribute(Attribute::Bold),
                    Cell::new("Aitiologia").add_attribute(Attribute::Bold),
                ]);

            let current_card_index = card_index + 1;

            card_detail_table.add_row(vec![
                Cell::new(format!("{}:{current_card_index}", card_detail.f_afm)),
                Cell::new(format!("{}:{current_card_index}", card_detail.f_eponymo)),
                Cell::new(format!("{}:{current_card_index}", card_detail.f_onoma)),
                Cell::new(format!("{}:{current_card_index}", card_detail.f_type)),
                Cell::new(format!(
                    "{}:{current_card_index}",
                    card_detail.f_reference_date
                )),
                Cell::new(format!("{}:{current_card_index}", card_detail.f_date)),
                Cell::new(format!("{}:{current_card_index}", card_detail.f_aitiologia)),
            ]);

            work_cards_tables.push(card_detail_table);
        }
    }

    for table in work_cards_tables {
        println!("{table}");
    }

    Ok(())
}
