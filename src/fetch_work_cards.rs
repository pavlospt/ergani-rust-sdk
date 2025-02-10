use anyhow::Result;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, ContentArrangement, Table};
use ergani::auth::authenticator::ErganiAuthenticationState;
use ergani::client::ErganiClient;

#[allow(dead_code)]
pub(crate) async fn fetch_work_cards(
    ergani_client: &ErganiClient,
    auth_state: ErganiAuthenticationState,
) -> Result<()> {
    let work_cards = ergani_client.fetch_work_cards(auth_state).await?;

    let mut work_cards_tables: Vec<Table> = vec![];

    for work_card in work_cards.response().unwrap().work_cards.cards.card.iter() {
        let mut work_card_table = Table::new();
        work_card_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("AFM Ergodoti").add_attribute(Attribute::Bold),
                Cell::new("AA").add_attribute(Attribute::Bold),
                Cell::new("Comments").add_attribute(Attribute::Bold),
            ]);

        work_card_table.add_row(vec![
            Cell::new(work_card.f_afm_ergodoti.to_string()),
            Cell::new(work_card.f_aa.to_string()),
            Cell::new(work_card.f_comments.to_string()),
        ]);

        work_cards_tables.push(work_card_table);

        let mut card_detail_table = Table::new();
        card_detail_table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("AFM").add_attribute(Attribute::Bold),
                Cell::new("Eponymo").add_attribute(Attribute::Bold),
                Cell::new("Onoma").add_attribute(Attribute::Bold),
                Cell::new("Type").add_attribute(Attribute::Bold),
                Cell::new("Reference Date").add_attribute(Attribute::Bold),
                Cell::new("Date").add_attribute(Attribute::Bold),
                Cell::new("Aitiologia").add_attribute(Attribute::Bold),
            ]);

        for card_detail in work_card.details.card_details.iter() {
            card_detail_table.add_row(vec![
                Cell::new(card_detail.f_afm.to_string()),
                Cell::new(card_detail.f_eponymo.to_string()),
                Cell::new(card_detail.f_onoma.to_string()),
                Cell::new(card_detail.f_type.to_string()),
                Cell::new(card_detail.f_reference_date.to_string()),
                Cell::new(card_detail.f_date.to_string()),
                Cell::new(card_detail.f_aitiologia.to_string()),
            ]);
        }

        work_cards_tables.push(card_detail_table);
    }

    for table in work_cards_tables {
        println!("{table}");
    }

    Ok(())
}
