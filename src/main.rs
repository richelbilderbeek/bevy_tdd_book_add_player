use crate::app::create_app;
mod app;

fn main() {
    let mut app = create_app();
    app.add_plugins(DefaultPlugins);
    app.run();
}
