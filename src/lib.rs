//! Anfegeth 1.0 is a bin package to perform a handshake with a local P2P Ethereum node.
//! You just need to set in .env URL (e.g http://localhost:8080) and NETWORK_FILE (e.g "network.json") this last one with JSON
//! containing the available ethereum network according to the central node in use. the JSON that MUST have the structure type "Network { id: String, name: String }"

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use web3::transports::Http;
use web3::Web3;

#[derive(Deserialize, Serialize, Debug)]
struct Network {
    id: String,
    name: String,
}

fn validate_handshake(network_id: &String, our_network_id: &String) -> Result<(), std::io::Error> {
    // Compare network IDs and protocol versions
    if our_network_id != network_id {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Incompatible network IDs",
        ));
    }
    Ok(())
}

fn get_stream(url: &str) -> Result<Web3<Http>, web3::Error> {
    let transport = Http::new(url)?;
    Ok(Web3::new(transport))
}

/// Returns the selected main menu item as a string
///
/// The main menu is a list of options that the user can choose from. This function prompts the user to select an option from the menu and returns the selected ID network option as a string.
///
/// # Examples
///
/// ```
/// let selected_option = get_sel_main_menu();
/// println!("You selected: {}", selected_option);
/// ```
///
/// # Errors
///
/// If the user enters an invalid option, the function will show a Warning message.
///
/// # Panics
///
/// This function may panic if the list of networks cannot be parsed as a JSON or Failed to read user input line.
pub fn get_sel_main_menu() -> String {
    dotenv().ok();
    let net_file = std::env::var("NETWORK_FILE").expect("Unable to acces ENV variable");
    let mut input = String::new();
    let data_net = fs::read_to_string(net_file).expect("Unable to read file");
    let networks: Vec<Network> = serde_json::from_str(&data_net).expect("Unable to parse JSON");

    loop {
        input.clear();

        println!("--- Welcome to ANFEGETH 1.0 Handshake ---");
        println!("Select Geth Network Node P2P ID to perform handshake:");
        for (i, net) in networks.iter().enumerate() {
            println!("{}. {}", (i + 1), net.name);
        }
        println!("Enter your choice or 'q' to exit:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            println!("Handshake Aborted!");
            break;
        }

        let choice: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if choice <= networks.len() && choice > 0{
            return networks[choice - 1].id.clone();
        } else {
            eprintln!("Chosen option doesn't exist")
        }
    }
    input
}

/// This is an async function that runs the main logic of the program
///
/// It takes a `String` representing the network ID as an argument and returns a `Result` with the `anyhow` crate.
///
/// # Examples
///
/// ```
/// use my_crate::run;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     run("mainnet".to_string()).await
/// }
/// ```
#[tokio::main]
pub async fn run(our_network_id: String) -> anyhow::Result<()> {
    dotenv().ok();
    let mut input = String::new();
    let url = std::env::var("URL")?;
    let web3 = get_stream(&url)?;

    loop {
        let network_id = web3.net().version().await?;
        validate_handshake(&network_id, &our_network_id)?;
        let is_listening = web3.net().is_listening().await?;
        println!("Network ID: {}, listening: {}", network_id, is_listening);
        let chain_id = web3.eth().chain_id().await?;
        println!("Chain ID: {}", chain_id);
        let gas_price = web3.eth().gas_price().await?;
        println!("Gas Price Estimated: {}", gas_price);
        let block_number = web3.eth().block_number().await?;
        println!("Block Number: {}", block_number);
        let client_version = web3.web3().client_version().await?;
        println!("Client Version: {}", client_version);
        let peer_count = web3.net().peer_count().await?;
        println!("Peer Count: {}", peer_count);

        println!("Press 'q' to exit or just Enter to update the information of the node: ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "q" {
            println!("Thanks for the handshake!");
            break;
        } else  {
            continue;
        }
    }
    Ok(())
}

mod test;
