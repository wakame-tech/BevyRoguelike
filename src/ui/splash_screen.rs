use crate::prelude::*;

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

fn splash_screen(
    mut commands: Commands,
    font_manager: Res<FontManager>,
    turn_state: Res<State<TurnState>>,
    top_ui_node_q: Query<Entity, With<TopUINode>>,
) {
    // If we are not in StartScreen we need to remove ALL the other UI stuff around the game
    if turn_state.0 != TurnState::StartScreen {
        let top_ui_node = top_ui_node_q.single();
        commands.entity(top_ui_node).despawn_recursive();
    }

    commands
    .spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..Default::default()
        },
        background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(MenuUI)
    .with_children(|parent| {

        // chose title based on State
        let mut title = "";
        let mut title_color = Color::GOLD;
        if turn_state.0 == TurnState::StartScreen {
            title = "Rogue Quest";
        } else if turn_state.0 == TurnState::GameOver {
            title = "Game Over";
            title_color = Color::RED;
        } else if turn_state.0 == TurnState::Victory {
            title = "You win!";
        } else if turn_state.0 == TurnState::NextLevel {
            title = "Level Completed";
        }

        // Spawn menu text
        parent.spawn(TextBundle {
            style: Style {
                size: Size::new(Val::Auto, Val::Px(140. * 1.)),
                margin: UiRect {
                    left: Val::Auto,
                    right: Val::Auto,
                    bottom: Val::Auto,
                    top: Val::Auto,
                },
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: title.to_string(),
                        style: TextStyle {
                            font: font_manager.font.clone(),
                            font_size: 100.0,
                            color: title_color,
                        },
                    },
                    TextSection {
                        value: "\nPress any key to start game.".to_string(),
                        style: TextStyle {
                            font: font_manager.font.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..Default::default()
        });
    });
}

// function to kill the current menu screen
fn despawn_splashscreen(
    mut commands: Commands, 
    query_startscreen: Query<Entity, With<MenuUI>>,
    query_gameoverscreen: Query<Entity, With<GameOverUI>>,
) {
    for e in query_startscreen.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_gameoverscreen.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn start_screen_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    turn_state: ResMut<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>
) {

    let key = keyboard_input.get_pressed().next().cloned();

    if let Some(key) = key {
        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);
        // update state
        if (turn_state.0 == TurnState::StartScreen) || 
            (turn_state.0 == TurnState::NextLevel) 
        {
            next_state.set(TurnState::AwaitingInput);
        } else {
            next_state.set(TurnState::StartScreen);
        }
        
    }
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // setup when entering the start screen
            .add_system(splash_screen.in_schedule(OnEnter(TurnState::StartScreen)))
            // setup when on the start screen
            .add_system(start_screen_input.in_set(OnUpdate(TurnState::StartScreen)))
            // cleanup when exiting the start screen
            .add_system(despawn_splashscreen.in_schedule(OnExit(TurnState::StartScreen)))

            // setup when entering the gameover screen
            .add_system(splash_screen.in_schedule(OnEnter(TurnState::GameOver)))
            // setup when on the gameover screen
            .add_system(start_screen_input.in_set(OnUpdate(TurnState::GameOver)))
            // cleanup when exiting the gameover screen
            .add_system(despawn_splashscreen.in_schedule(OnExit(TurnState::GameOver)))


            // setup when entering the victory screen
            .add_system(splash_screen.in_schedule(OnEnter(TurnState::Victory)))
            // setup when on the victory screen
            .add_system(start_screen_input.in_set(OnUpdate(TurnState::Victory)))
            // cleanup when exiting the victory screen
            .add_system(despawn_splashscreen.in_schedule(OnExit(TurnState::Victory)))

            // setup when entering the next level screen
            .add_system(splash_screen.in_schedule(OnEnter(TurnState::NextLevel)))
            // setup when on the next level screen
            .add_system(start_screen_input.in_set(OnUpdate(TurnState::NextLevel)))
            // cleanup when exiting the next level screen
            .add_system(despawn_splashscreen.in_schedule(OnExit(TurnState::NextLevel)));
    }
}