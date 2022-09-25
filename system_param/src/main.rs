use bevy::{ecs::system::SystemParam, prelude::*};

fn main() {
    println!("Hello, System Param!");
    App::new()
        .insert_resource(PlayerCount(0))
        .add_startup_system(spawn)
        .add_system(count_players)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCount(usize);


#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    players: Query<'w, 's, &'static Player>,
    count: ResMut<'w, PlayerCount>,
}

impl<'s, 'w> PlayerCounter<'s, 'w> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

fn spawn(mut commands: Commands) {
    commands.spawn().insert(Player);
    commands.spawn().insert(Player);
    commands.spawn().insert(Player);
}

fn count_players(mut counter: PlayerCounter) {
    counter.count();

    println!("{} players in game", counter.count.0);
}
