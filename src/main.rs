use bevy::{
    core::FixedTimestep,
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    render::camera::Camera,
    sprite::Rect,
};

const TOTAL_AMMO: usize = 3;
const CROSSHAIR_OFFSET_X: f32 = 100.;
const CROSSHAIR_OFFSET_Y: f32 = 200.;
const TOTAL_TIME: usize = 90;

struct Textures {
    sprites_stall: Handle<TextureAtlas>,
    sprites_hud: Handle<TextureAtlas>,
    sprites_objects: Handle<TextureAtlas>,
}
#[derive(Default)]
struct Game {
    score: usize,
    time_left: usize,
    ammo: usize,
    last_mouse: Vec2,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum GameState {
    Playing,
    GameOver,
}

struct FrontWave;
struct BackWave;
struct Cloud;

enum TimeKind {
    Minute,
    Colon,
    Ten,
    Second,
}
struct Clock {
    kind: TimeKind,
}
enum ScoreKind {
    Thousand,
    Hundred,
    Ten,
    One,
}
struct Score {
    kind: ScoreKind,
}

struct Crosshair;
struct Rifle;
struct Target {
    speed: f32,
    points: usize,
    hit_box: f32,
    is_up_down: bool,
    start_y: f32,
}
struct TargetStick {
    speed: f32,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Take a Shot!".to_string(),
            resizable: false,
            cursor_visible: false,
            ..Default::default()
        })
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_stall.system())
                .with_system(setup_rifle.system())
                .with_system(setup_targets.system())
                .with_system(setup_hud.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(mouse_move_events.system())
                .with_system(mouse_button_events.system())
                .with_system(update_score.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown.system()))
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(display_score.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(gameover_keyboard.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown.system()))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(animate_stall.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(count_down.system()),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(animate_targets.system()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    game.score = 0;
    game.time_left = TOTAL_TIME;
    game.ammo = TOTAL_AMMO;
    let stall_texture_handle = asset_server.load("textures/spritesheet_stall.png");
    let mut stall_texture_atlas =
        TextureAtlas::new_empty(stall_texture_handle, Vec2::new(794., 802.));
    // straight curtain
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(0., 0.),
        max: Vec2::new(256., 80.),
    });
    // top curtain
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(0., 598.),
        max: Vec2::new(200., 598. + 63.),
    });
    // side curtain
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(650., 0.),
        max: Vec2::new(650. + 131., 426.),
    });
    // side curtain rope
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(0., 762.),
        max: Vec2::new(40., 762. + 21.),
    });
    // wood bg
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(258., 0.),
        max: Vec2::new(258. + 256., 256.),
    });
    // grass 1
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(403., 600.),
        max: Vec2::new(403. + 132., 600. + 200.),
    });
    // grass 2
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(516., 0.),
        max: Vec2::new(516. + 132., 216.),
    });
    // water 1
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(516., 218.),
        max: Vec2::new(516. + 132., 218. + 224.),
    });
    // water 2
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(539., 444.),
        max: Vec2::new(539. + 132., 444. + 223.),
    });
    // cloud 1
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(403., 516.),
        max: Vec2::new(403. + 134., 516. + 82.),
    });
    // cloud 2
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(0., 663.),
        max: Vec2::new(141., 663. + 84.),
    });
    // tree oak
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(258., 516.),
        max: Vec2::new(258. + 143., 516. + 244.),
    });
    // tree pine
    stall_texture_atlas.add_texture(Rect {
        min: Vec2::new(673., 428.),
        max: Vec2::new(673. + 119., 428. + 255.),
    });

    let hud_texture_handle = asset_server.load("textures/spritesheet_hud.png");
    let mut hud_texture_atlas = TextureAtlas::new_empty(hud_texture_handle, Vec2::new(418., 421.));
    // number 0
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(303., 382.),
        max: Vec2::new(303. + 32., 382. + 37.),
    });
    // number 1
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(382., 0.),
        max: Vec2::new(382. + 23., 36.),
    });
    // number 2
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(359., 271.),
        max: Vec2::new(359. + 29., 271. + 37.),
    });
    // number 3
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(365., 111.),
        max: Vec2::new(365. + 28., 111. + 36.),
    });
    // number 4
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(359., 233.),
        max: Vec2::new(359. + 30., 233. + 36.),
    });
    // number 5
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(359., 310.),
        max: Vec2::new(359. + 29., 310. + 36.),
    });
    // number 6
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(370., 378.),
        max: Vec2::new(370. + 28., 378. + 36.),
    });
    // number 7
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(340., 73.),
        max: Vec2::new(340. + 30., 73. + 36.),
    });
    // number 8
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(337., 378.),
        max: Vec2::new(337. + 31., 378. + 37.),
    });
    // number 9
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(350., 0.),
        max: Vec2::new(350. + 30., 36.),
    });
    // colon
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(34., 387.),
        max: Vec2::new(34. + 21., 387. + 32.),
    });
    // cross
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(0., 387.),
        max: Vec2::new(32., 387. + 32.),
    });
    // plus
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(359., 348.),
        max: Vec2::new(359. + 28., 348. + 28.),
    });
    // crosshair
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(195., 212.),
        max: Vec2::new(195. + 50., 212. + 50.),
    });
    // crosshair fired
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(169., 330.),
        max: Vec2::new(169. + 50., 330. + 50.),
    });
    // score
    hud_texture_atlas.add_texture(Rect {
        min: Vec2::new(0., 278.),
        max: Vec2::new(116., 278. + 39.),
    });

    let obj_texture_handle = asset_server.load("textures/spritesheet_objects.png");
    let mut obj_texture_atlas = TextureAtlas::new_empty(obj_texture_handle, Vec2::new(736., 736.));
    // rifle
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(144., 0.),
        max: Vec2::new(144. + 142., 319.),
    });
    // rifle fired
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(288., 0.),
        max: Vec2::new(288. + 141., 319.),
    });
    // duck target yellow
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(547., 0.),
        max: Vec2::new(547. + 99., 95.),
    });
    // stick wood
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(650., 258.),
        max: Vec2::new(650. + 34., 258. + 127.),
    });
    // stick metal
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(648., 0.),
        max: Vec2::new(648. + 34., 127.),
    });
    // target colored
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(144., 595.),
        max: Vec2::new(144. + 128., 595. + 128.),
    });
    // target red1
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(404., 451.),
        max: Vec2::new(404. + 128., 451. + 128.),
    });
    // duck target brown
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(636., 638.),
        max: Vec2::new(636. + 99., 638. + 95.),
    });
    // target white
    obj_texture_atlas.add_texture(Rect {
        min: Vec2::new(288., 321.),
        max: Vec2::new(288. + 128., 321. + 128.),
    });

    commands
        .spawn(OrthographicCameraBundle::new_2d())
        .spawn(UiCameraBundle::default())
        .insert_resource(Textures {
            sprites_stall: texture_atlases.add(stall_texture_atlas),
            sprites_hud: texture_atlases.add(hud_texture_atlas),
            sprites_objects: texture_atlases.add(obj_texture_atlas),
        })
        .with(Timer::from_seconds(0.1, true));
}

fn setup_stall(mut commands: Commands, texture: Res<Textures>) {
    // setup primary top curtain
    let mut curtain_primary_start = -512.;
    while curtain_primary_start <= 512. {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_stall.clone(),
            transform: Transform::from_xyz(curtain_primary_start, 80. * 4., 2.),
            ..Default::default()
        });
        curtain_primary_start += 256.;
    }
    // setup secondary top curtain
    let mut curtain_secondary_start = -540.;
    while curtain_secondary_start <= 540. {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_stall.clone(),
            transform: Transform::from_xyz(curtain_secondary_start, 63. * 4.3, 1.),
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            ..Default::default()
        });
        curtain_secondary_start += 180.;
    }
    // setup side curtains
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_stall.clone(),
        transform: Transform {
            translation: Vec3::new(-582., 100., 1.9),
            scale: Vec3::splat(1.3),
            ..Default::default()
        },
        sprite: TextureAtlasSprite {
            index: 2,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_stall.clone(),
        transform: Transform {
            translation: Vec3::new(582., 100., 1.9),
            scale: Vec3::splat(1.3),
            ..Default::default()
        },
        sprite: TextureAtlasSprite {
            index: 2,
            flip_x: true,
            ..Default::default()
        },
        ..Default::default()
    });
    // setup side rope
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_stall.clone(),
        transform: Transform {
            translation: Vec3::new(-640., 92., 1.95),
            scale: Vec3::splat(1.3),
            ..Default::default()
        },
        sprite: TextureAtlasSprite {
            index: 3,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_stall.clone(),
        transform: Transform {
            translation: Vec3::new(640., 92., 1.95),
            scale: Vec3::splat(1.3),
            ..Default::default()
        },
        sprite: TextureAtlasSprite {
            index: 3,
            flip_x: true,
            ..Default::default()
        },
        ..Default::default()
    });
    // setup bottom wood frame
    let mut bottom_frame_start = -384.;
    while bottom_frame_start <= 384. {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_stall.clone(),
            transform: Transform {
                translation: Vec3::new(bottom_frame_start, -395., 1.8),
                scale: Vec3::splat(2.),
                ..Default::default()
            },
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            ..Default::default()
        });
        bottom_frame_start += 256.;
    }
    // setup background wood
    let mut bg_wood_start = -512.;
    while bg_wood_start <= 512. {
        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_stall.clone(),
            transform: Transform::from_xyz(bg_wood_start, 200., 1.7),
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            ..Default::default()
        });
        bg_wood_start += 256.;
    }
    // setup grass
    for i in 0..11 {
        let offset = -660. + i as f32 * 132.;
        if i % 2 == 0 {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture.sprites_stall.clone(),
                transform: Transform {
                    translation: Vec3::new(offset, 8., 1.73),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 6,
                    flip_x: true,
                    ..Default::default()
                },
                ..Default::default()
            });
        } else {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture.sprites_stall.clone(),
                transform: Transform {
                    translation: Vec3::new(offset, 0., 1.73),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index: 5,
                    flip_x: true,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
    // setup back water
    let mut back_water_start = -660.;
    while back_water_start <= 660. {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture.sprites_stall.clone(),
                transform: Transform::from_xyz(back_water_start, -90., 1.75),
                sprite: TextureAtlasSprite {
                    index: 7,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(BackWave);
        back_water_start += 132.;
    }
    // setup front water
    let mut front_water_start = -620.;
    while front_water_start <= 620. {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture.sprites_stall.clone(),
                transform: Transform::from_xyz(front_water_start, -120., 1.78),
                sprite: TextureAtlasSprite {
                    index: 8,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(FrontWave);
        front_water_start += 132.;
    }
    // add cloud 1
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_stall.clone(),
            transform: Transform::from_xyz(-300., 220., 1.71),
            sprite: TextureAtlasSprite {
                index: 9,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Cloud);
    // add cloud 2
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_stall.clone(),
            transform: Transform::from_xyz(300., 260., 1.71),
            sprite: TextureAtlasSprite {
                index: 10,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Cloud);
    // add oak tree
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_stall.clone(),
        transform: Transform::from_xyz(-530., 190., 1.71),
        sprite: TextureAtlasSprite {
            index: 11,
            ..Default::default()
        },
        ..Default::default()
    });
    // add pine tree
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_stall.clone(),
        transform: Transform::from_xyz(530., 130., 1.73),
        sprite: TextureAtlasSprite {
            index: 12,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn setup_hud(mut commands: Commands, texture: Res<Textures>) {
    // setup timer
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(-600., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 1,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Clock {
            kind: TimeKind::Minute,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(-600. + 28., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 10,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Clock {
            kind: TimeKind::Colon,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(-600. + 56., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 3,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Clock {
            kind: TimeKind::Ten,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(-600. + 84., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Clock {
            kind: TimeKind::Second,
        });
    // setup score
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_hud.clone(),
        transform: Transform::from_xyz(400., 330., 3.),
        sprite: TextureAtlasSprite {
            index: 15,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture.sprites_hud.clone(),
        transform: Transform::from_xyz(470., 330., 3.),
        sprite: TextureAtlasSprite {
            index: 10,
            ..Default::default()
        },
        ..Default::default()
    });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(470. + 28., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Score {
            kind: ScoreKind::Thousand,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(470. + 56., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Score {
            kind: ScoreKind::Hundred,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(470. + 84., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Score {
            kind: ScoreKind::Ten,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(470. + 112., 330., 3.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Score {
            kind: ScoreKind::One,
        });
}

fn setup_rifle(mut commands: Commands, texture: Res<Textures>) {
    // setup rifle
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(CROSSHAIR_OFFSET_X, -CROSSHAIR_OFFSET_Y, 4.),
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Rifle);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_hud.clone(),
            transform: Transform::from_xyz(0., 0., 4.),
            sprite: TextureAtlasSprite {
                index: 13,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Crosshair);
}

fn setup_targets(mut commands: Commands, texture: Res<Textures>) {
    // setup duck
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(-300., 50., 1.77),
            sprite: TextureAtlasSprite {
                index: 2,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Target {
            speed: 3.,
            points: 10,
            hit_box: 99.,
            is_up_down: false,
            start_y: 0.,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(-305., -55., 1.76),
            sprite: TextureAtlasSprite {
                index: 3,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(TargetStick { speed: 3. });
    // setup brown duck
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(300., 70., 1.74),
            sprite: TextureAtlasSprite {
                index: 7,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Target {
            speed: 15.,
            points: 20,
            hit_box: 99.,
            is_up_down: false,
            start_y: 0.,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(295., -35., 1.73),
            sprite: TextureAtlasSprite {
                index: 3,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(TargetStick { speed: 15. });
    // setup colored target
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(0., 183., 1.72),
            sprite: TextureAtlasSprite {
                index: 5,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Target {
            speed: 25.,
            points: 25,
            hit_box: 128.,
            is_up_down: false,
            start_y: 0.,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(0., 60., 1.71),
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(TargetStick { speed: 25. });
    // setup red target
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(300., 203., 1.72),
            sprite: TextureAtlasSprite {
                index: 6,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Target {
            speed: 50.,
            points: 50,
            hit_box: 128.,
            is_up_down: false,
            start_y: 0.,
        });
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(300., 80., 1.71),
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(TargetStick { speed: 50. });
    // setup white target
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture.sprites_objects.clone(),
            transform: Transform::from_xyz(-300., 120., 1.72),
            sprite: TextureAtlasSprite {
                index: 8,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Target {
            speed: 100.,
            points: 250,
            hit_box: 128.,
            is_up_down: true,
            start_y: 120.,
        });
}

fn mouse_move_events(
    mut transforms: QuerySet<(
        Query<&mut Transform, With<Rifle>>,
        Query<&mut Transform, With<Crosshair>>,
    )>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut game: ResMut<Game>,
) {
    for event in cursor_moved_events.iter() {
        let new_x = event.position.x - 1280. / 2.;
        let new_y = event.position.y - 720. / 2.;
        for mut transform in transforms.q0_mut().iter_mut() {
            transform.translation.x = new_x + CROSSHAIR_OFFSET_X;
            transform.translation.y = new_y - CROSSHAIR_OFFSET_Y;
        }
        for mut transform in transforms.q1_mut().iter_mut() {
            transform.translation.x = new_x;
            transform.translation.y = new_y;
            game.last_mouse = Vec2::new(new_x, new_y);
        }
    }
}

fn is_hit(mouse: Vec2, target: Vec2, hit_box: f32) -> bool {
    let mut is_x_hit = false;
    let mut is_y_hit = false;
    let half_box = hit_box / 2.;
    if mouse.x > target.x - half_box && mouse.x < target.x + half_box {
        is_x_hit = true;
    }
    if mouse.y < target.y + half_box && mouse.y > target.y - half_box {
        is_y_hit = true;
    }
    is_x_hit && is_y_hit
}

fn mouse_button_events(
    mut transforms: QuerySet<(
        Query<(&Transform, &Target)>,
        Query<&mut TextureAtlasSprite, With<Rifle>>,
        Query<&mut TextureAtlasSprite, With<Crosshair>>,
    )>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut game: ResMut<Game>,
) {
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left && event.state == ElementState::Pressed {
            for (transform, target) in transforms.q0_mut().iter_mut() {
                if is_hit(
                    game.last_mouse,
                    Vec2::new(transform.translation.x, transform.translation.y),
                    target.hit_box,
                ) {
                    game.score += target.points;
                }
            }
            for mut sprite in transforms.q1_mut().iter_mut() {
                sprite.index = 1;
            }
            for mut sprite in transforms.q2_mut().iter_mut() {
                sprite.index = 14;
            }
        } else {
            for mut sprite in transforms.q1_mut().iter_mut() {
                sprite.index = 0;
            }
            for mut sprite in transforms.q2_mut().iter_mut() {
                sprite.index = 13;
            }
        }
    }
}

fn animate_stall(
    state: Res<State<GameState>>,
    mut transforms: QuerySet<(
        Query<&mut Transform, With<Cloud>>,
        Query<&mut Transform, With<BackWave>>,
        Query<&mut Transform, With<FrontWave>>,
    )>,
) {
    if *state.current() != GameState::Playing {
        return;
    }

    for mut transform in transforms.q0_mut().iter_mut() {
        transform.translation.x += 1.;

        if transform.translation.x > 650. {
            transform.translation.x = -650.
        }
    }
    for mut transform in transforms.q1_mut().iter_mut() {
        transform.translation.x -= 2.;

        if transform.translation.x < -660. {
            transform.translation.x = 660.
        }
    }
    for mut transform in transforms.q2_mut().iter_mut() {
        transform.translation.x += 2.;

        if transform.translation.x > 660. {
            transform.translation.x = -655.
        }
    }
}

fn animate_targets(
    state: Res<State<GameState>>,
    mut transforms: QuerySet<(
        Query<(&mut Transform, &Target)>,
        Query<(&mut Transform, &TargetStick)>,
    )>,
) {
    if *state.current() != GameState::Playing {
        return;
    }

    for (mut transform, target) in transforms.q0_mut().iter_mut() {
        transform.translation.x += target.speed;

        if transform.translation.x > 650. {
            transform.translation.x = -650.
        }

        if target.is_up_down {
            if target.start_y == transform.translation.y {
                transform.translation.y = target.start_y / 2.;
            } else {
                transform.translation.y = target.start_y;
            }
        }
    }
    for (mut transform, target) in transforms.q1_mut().iter_mut() {
        transform.translation.x += target.speed;

        if transform.translation.x > 650. {
            transform.translation.x = -650.
        }
    }
}

fn count_down(
    mut state: ResMut<State<GameState>>,
    mut query: Query<(&mut TextureAtlasSprite, &Clock)>,
    mut game: ResMut<Game>,
) {
    if *state.current() != GameState::Playing {
        return;
    }

    if game.time_left == 0 {
        state.set_next(GameState::GameOver).unwrap();
        return;
    }
    game.time_left -= 1;
    for (mut sprite, clock) in query.iter_mut() {
        match clock.kind {
            TimeKind::Minute => {
                if game.time_left >= 60 {
                    sprite.index = 1;
                } else {
                    sprite.index = 0;
                }
            }
            TimeKind::Ten => {
                let time_left = game.time_left as u32 % 60;
                sprite.index = (time_left - time_left as u32 % 10) % 100 / 10;
            }
            TimeKind::Second => {
                sprite.index = game.time_left as u32 % 10;
            }
            _ => {}
        }
    }
}

fn update_score(mut query: Query<(&mut TextureAtlasSprite, &Score)>, game: Res<Game>) {
    for (mut sprite, digit) in query.iter_mut() {
        let score = game.score as u32;
        match digit.kind {
            ScoreKind::Thousand => {
                sprite.index = (score - score as u32 % 1000) % 10000 / 1000;
            }
            ScoreKind::Hundred => {
                sprite.index = (score - score as u32 % 100) % 1000 / 100;
            }
            ScoreKind::Ten => {
                sprite.index = (score - score as u32 % 10) % 100 / 10;
            }
            ScoreKind::One => {
                sprite.index = score as u32 % 10;
            }
        }
    }
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in entities.iter() {
        commands.despawn_recursive(entity);
    }
}

// restart the game when pressing spacebar
fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set_next(GameState::Playing).unwrap();
    }
}

// display the final score
fn display_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: bevy::math::Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: format!("Final Score: {}", game.score),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.0, 1.0, 0.0),
                            },
                        },
                        TextSection {
                            value: "\nYou Won! Press Spacebar to Play Again".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.0, 1.0, 0.0),
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    game.score = 0;
    game.time_left = TOTAL_TIME;
}
