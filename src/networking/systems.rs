use bevy::prelude::*;
use std::net::SocketAddr;
use structopt::StructOpt;
use crate::networking::resources::*;

// structopt reads commandline parameters
#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, default_value = "0")]
    local_port: u16,
    #[structopt(short, long)]
    players: Vec<String>,
    #[structopt(short, long)]
    spectators: Vec<SocketAddr>,
}

pub fn setup_session(mut commands: Commands) {
    let opt = Opt::from_args();
    let num_players = opt.players.len();
    assert!(num_players < 3, "\n\nYou specified more than two players. I'm afraid the game can't handle that :(\n\n");

    let session: GameSessionType = match num_players {
        2 => GameSessionType::LAN,
        _ => GameSessionType::Local
    };
    commands.insert_resource(session);
}