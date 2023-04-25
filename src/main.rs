
use twitch_irc::{login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient, message};

use crate::vista::ui::app;
const APP_ID: &str = "com.ruleta_gtk-rs.app";

#[tokio::main]
pub async fn main(){
        app().await;
}



fn message_recived(msg: message::PrivmsgMessage){
    println!("Mensaje Recibido de {}: {}:", msg.sender.name, msg.message_text);
}
pub async fn connection() {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);


    let join_handle: tokio::task::JoinHandle<()> = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                message::ServerMessage::Privmsg(msg) => {
                    if msg.message_text.contains("!play") {
                        message_recived(msg);
                    }
                }
                _ => {}
            }

        }
    });

    client.join("al_css_".to_owned()).unwrap();
    println!("Escuchando...");


    join_handle.await.unwrap();
}
