use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_systems(Startup, add_player);
    app.update();
    app
}
fn add_player(mut commands: Commands) {
    commands.spawn(Player);
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    query.iter(app.world()).len()
}


#[cfg(test)]
fn is_bevy_component<T: Component>(_t: T) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        app.update();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_player_class_exists() {
        let player: Player;
    }
    #[test]
    fn test_player_class_has_a_name() {
        use std::any::type_name;
        assert!(type_name::<Player>().contains("Player"));
    }
    #[test]
    fn test_player_class_is_a_bevy_component() {
        let player = Player;
        assert!(is_bevy_component(player));
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        assert_eq!(count_n_players(&mut app), 1);
    }
}
