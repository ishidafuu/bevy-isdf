use bevy::prelude::*;

pub struct SwitchStatePlugin;
impl Plugin for SwitchStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_state(AppStates::Setup)
            //Setup sequence
            .add_system_set(SystemSet::on_enter(AppStates::Setup).with_system(on_setup_entry))
            .add_system_set(SystemSet::on_update(AppStates::Setup).with_system(on_setup_process))
            .add_system_set(SystemSet::on_exit(AppStates::Setup).with_system(on_setup_exit))
            //State A
            .add_system_set(SystemSet::on_enter(AppStates::A).with_system(on_state_a_entered))
            .add_system_set(SystemSet::on_update(AppStates::A).with_system(on_state_a_process))
            .add_system_set(SystemSet::on_exit(AppStates::A).with_system(on_state_a_exit))
            //State B
            .add_system_set(SystemSet::on_enter(AppStates::B).with_system(on_state_b_entered))
            .add_system_set(SystemSet::on_update(AppStates::B).with_system(on_state_b_process))
            .add_system_set(SystemSet::on_exit(AppStates::B).with_system(on_state_b_exit));
    }
}

#[derive(Component)]
pub struct SceneBoard;
#[derive(Component)]
pub struct SceneHeader;
#[derive(Component)]
pub struct SceneDescription;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppStates {
    Setup,
    A,
    B,
}

pub fn setup(mut commands: Commands, loader: Res<AssetServer>) {
    let font = loader.load("fonts/Picory.ttf");
    let text_style = TextStyle {
        color: Color::WHITE,
        font: font.clone(),
        font_size: 48.,
    };
    commands.insert_resource(text_style.clone());
    commands.insert_resource(Time::default());
    commands.insert_resource(Timer::from_seconds(10., true));
    commands
        .spawn_bundle(ImageBundle {
            color: UiColor(Color::RED),
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                position_type: PositionType::Absolute,
                position: UiRect::<Val> {
                    top: Val::Percent(15.),
                    bottom: Val::Percent(15.),
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SceneBoard)
        .with_children(|builder| {
            builder
                .spawn_bundle(TextBundle {
                    text: Text {
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SceneDescription);
            builder
                .spawn_bundle(TextBundle {
                    text: Text {
                        alignment: TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(SceneHeader);
        });
}

pub fn on_setup_entry(
    mut header_q: Query<(Entity, &mut Text), With<SceneHeader>>,
    mut timer: ResMut<Timer>,
    ts: Res<TextStyle>,
) {
    eprintln!("Setup sequence start!!");
    timer.reset();

    let (_, mut text) = header_q
        .get_single_mut()
        .expect("No header text element in the board.");
    text.sections = vec![TextSection {
        value: "Setup Sequence".to_string(),
        style: ts.clone(),
    }];
}

pub fn on_setup_process(
    mut description_q: Query<(Entity, &mut Text), With<SceneDescription>>,
    mut states: ResMut<State<AppStates>>,
    time: ResMut<Time>,
    mut timer: ResMut<Timer>,
    ts: Res<TextStyle>,
) {
    let (_, mut text) = description_q
        .get_single_mut()
        .expect("No header text element in the board.");
    let time_text = format!("Time:{:.2}", timer.elapsed_secs());
    text.sections = vec![TextSection {
        value: time_text,
        style: ts.clone(),
    }];

    timer.tick(time.delta());
    if timer.elapsed().as_secs() >= 5 {
        states.set(AppStates::A).expect("TODO: panic message");
    }
}
pub fn on_setup_exit() {
    eprintln!("Setup finished.")
}

pub fn on_state_a_entered(
    mut header_q: Query<(Entity, &mut Text), With<SceneHeader>>,
    mut timer: ResMut<Timer>,
    ts: Res<TextStyle>,
) {
    eprintln!("A sequnce start!!");
    timer.reset();

    let (_, mut text) = header_q
        .get_single_mut()
        .expect("No header text element in the board.");
    text.sections = vec![TextSection {
        value: "State A".to_string(),
        style: ts.clone(),
    }];
}

pub fn on_state_a_process(
    mut description_q: Query<(Entity, &mut Text), With<SceneDescription>>,
    mut states: ResMut<State<AppStates>>,
    time: ResMut<Time>,
    mut timer: ResMut<Timer>,
    ts: Res<TextStyle>,
) {
    let (_, mut text) = description_q
        .get_single_mut()
        .expect("No SceneDescription text element in the board.");
    let time_text = format!("Time:{:.2}", timer.elapsed_secs());
    text.sections = vec![TextSection {
        value: time_text,
        style: ts.clone(),
    }];

    timer.tick(time.delta());
    if timer.elapsed().as_secs() >= 5 {
        states.set(AppStates::B).expect("TODO: panic message");
    }
}
pub fn on_state_a_exit() {
    eprintln!("A finished.")
}

pub fn on_state_b_entered(
    mut header_q: Query<(Entity, &mut Text), With<SceneHeader>>,
    mut timer: ResMut<Timer>,
    ts: Res<TextStyle>,
) {
    eprintln!("B sequnce start!!");
    timer.reset();
    let (_, mut text) = header_q
        .get_single_mut()
        .expect("No header text element in the board.");
    text.sections = vec![TextSection {
        value: "State B".to_string(),
        style: ts.clone(),
    }];
}

pub fn on_state_b_process(
    mut description_q: Query<(Entity, &mut Text), With<SceneDescription>>,
    mut states: ResMut<State<AppStates>>,
    time: ResMut<Time>,
    mut timer: ResMut<Timer>,
    ts: Res<TextStyle>,
) {
    let (_, mut text) = description_q
        .get_single_mut()
        .expect("No SceneDescription text element in the board.");
    let time_text = format!("Time:{:.2}", timer.elapsed_secs());
    text.sections = vec![TextSection {
        value: time_text,
        style: ts.clone(),
    }];

    timer.tick(time.delta());
    if timer.elapsed().as_secs() >= 5 {
        states.set(AppStates::A).expect("TODO: panic message");
    }
}
pub fn on_state_b_exit() {
    eprintln!("B finished.")
}
