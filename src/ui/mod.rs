use crate::prelude::*;

mod equipment;
mod hud;
mod inventory;
mod popup;
mod splash_screen;
mod tooltips;

#[derive(Component)]
pub struct TopUINode;

#[derive(Resource)]
pub(crate) struct FontManager {
    pub font: Handle<Font>,
}

fn setup(asset_server: ResMut<AssetServer>, mut commands: Commands) {
    let font: Handle<Font> = asset_server.load("fonts/dos.ttf");
    let manager = FontManager { font };
    commands.insert_resource(manager);
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_plugins(splash_screen::MenuPlugin)
            .add_plugins(hud::HudPlugin)
            .add_plugins(popup::PopUpPlugin)
            .add_plugins(tooltips::TooltipsPlugin);
    }
}
