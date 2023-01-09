use anfegeth::*;

pub fn main() -> anyhow::Result<()> {
    let our_network_id = get_sel_main_menu();
    if our_network_id != "q\n" {
        run(our_network_id)?;
    }

    Ok(())
}
