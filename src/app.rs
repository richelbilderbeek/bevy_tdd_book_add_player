use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;

/// Create a Bevy app
pub fn create_app() -> App {
    let mut app = App::new();
    app.add_systems(Startup, setup_paddle);
    app.update();
    return app;
}

// Add the paddle to our world
fn setup_paddle(mut commands: Commands) {
    commands.spawn((SpriteBundle { ..default() }, Paddle));
}

#[cfg(test)]
fn count_n_paddles(app: &App) -> usize {
    let mut n = 0;
    for c in app.world.components().iter() {
        // The complete name will be '[crate_name]::Paddle'
        if c.name().contains("Paddle") {
            n += 1;
        }
    }
    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testing() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn test_can_create_app() {
        create_app();
    }

    #[test]
    fn test_empty_app_has_no_paddles() {
        let app = App::new();
        assert_eq!(count_n_paddles(&app), 0);
    }

    #[test]
    fn test_setup_paddle_adds_a_paddle() {
        let mut app = App::new();
        assert_eq!(count_n_paddles(&app), 0);
        app.add_systems(Startup, setup_paddle);
        app.update();
        assert_eq!(count_n_paddles(&app), 1);
    }

    #[test]
    fn test_create_app_has_a_paddle() {
        let app = create_app();
        assert_eq!(count_n_paddles(&app), 1);
    }
}
