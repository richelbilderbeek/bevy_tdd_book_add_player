use bevy::prelude::*;

/// Create a Bevy app
pub fn create_app() -> App {
    return App::new();
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
}
