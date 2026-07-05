mod game_over;
mod game_pause;

use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

use crate::AppState;

#[derive(SubStates, Default, Debug, Hash, Eq, PartialEq, Clone)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct Lives(usize);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSystemSet {
    Preload,
    Input,
    Movement,
    Collision,
    PostCollision,
    Display,
}

pub fn plugin(app: &mut App) {
    app.configure_sets(
        OnEnter(AppState::MainMenu),
        (
            GameSystemSet::Preload,
            GameSystemSet::PostCollision,
            GameSystemSet::Display,
        )
            .chain(),
    );
    app.configure_sets(
        FixedUpdate,
        (
            GameSystemSet::Input,
            GameSystemSet::Movement,
            GameSystemSet::Collision,
            GameSystemSet::PostCollision,
            GameSystemSet::Display,
        )
            .chain(),
    );
    app.add_plugins(game_pause::plugin)
        .add_plugins(game_over::plugin)
        .add_systems(OnEnter(AppState::InGame), game_ui.spawn())
        .add_systems(Update, handle_input.run_if(in_state(GameState::Running)));
    app.add_systems(
        OnEnter(AppState::MainMenu),
        (
            spawn_entities.in_set(GameSystemSet::Preload),
            spawn_bricks.in_set(GameSystemSet::Preload),
            move_locked_ball.in_set(GameSystemSet::PostCollision),
            project_positions.in_set(GameSystemSet::Display),
            init_resources,
        ),
    )
    .add_systems(
        Update,
        (
            update_lives_display.run_if(resource_changed::<Lives>),
            update_score_display.run_if(resource_changed::<Score>),
        ),
    )
    .add_systems(
        FixedUpdate,
        (
            project_positions.in_set(GameSystemSet::Display),
            launch_ball.in_set(GameSystemSet::Input),
            move_ball.in_set(GameSystemSet::Movement),
            move_locked_ball.in_set(GameSystemSet::PostCollision),
            handle_collisions
                .in_set(GameSystemSet::Collision)
                .after(constrain_paddle_position),
            handle_player_input.in_set(GameSystemSet::Input),
            move_paddle.in_set(GameSystemSet::Movement),
            constrain_paddle_position.in_set(GameSystemSet::Collision),
            handle_lost_ball.in_set(GameSystemSet::Collision),
            set_win_state
                .run_if(not(any_with_component::<Brick>))
                .in_set(GameSystemSet::PostCollision)
                .ambiguous_with(check_out_of_lives),
            check_out_of_lives
                .run_if(resource_changed::<Lives>)
                .in_set(GameSystemSet::PostCollision),
        )
            .run_if(in_state(GameState::Running)),
    )
    .add_observer(destroy_brick);
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

#[derive(Component, Default, Clone, Copy)]
struct LivesDisplay;

#[derive(Component, Default, Clone, Copy)]
struct ScoreDisplay;

fn game_ui() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            // height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexEnd,
        }
        DespawnOnExit<AppState>(AppState::InGame)
        Children [
            (
                ScoreDisplay
                Text::new(format!("{} points", 0))
            ),
            (
                LivesDisplay
                Text::new(format!("{} lives", 3))
                // TextColor(Color::BLACK)
                // TextLayout::justify(Justify::Center)
            ),
        ]
    }
}

mod physics {
    use bevy::prelude::*;

    #[derive(Component, Clone, Default)]
    #[require(Transform)]
    pub struct Position(pub Vec2);

    #[derive(Component, Clone, Default)]
    pub struct Velocity(pub Vec2);

    #[derive(Component, Clone, Default)]
    pub struct Collider(pub Rectangle);

    impl Collider {
        pub fn half_size(&self) -> Vec2 {
            self.0.half_size
        }
    }

    pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
        for (mut transform, position) in &mut positionables {
            transform.translation = position.0.extend(0.);
        }
    }
}

use physics::{Collider, Position, Velocity, project_positions};

#[derive(EntityEvent)]
struct BrickDestroyed {
    entity: Entity,
}

#[derive(Resource)]
struct Score(u32);

#[derive(SceneComponent, Clone, Default)]
struct Ball;

const BALL_SIZE: f32 = 10.;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
const BALL_SPEED: f32 = 2.;
const BALL_BASE_POS: Vec2 = vec2(0., -200.);
const BALL_BASE_VELOCITY: Vec2 = vec2(0., BALL_SPEED);

impl Ball {
    fn scene() -> impl Scene {
        bsn! {
            Position(BALL_BASE_POS)
            Collider(Rectangle::new(BALL_SIZE, BALL_SIZE))
            Mesh2d(asset_value(BALL_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(BALL_COLOR))
            DespawnOnExit<AppState>(AppState::InGame)
        }
    }
}

#[derive(SceneComponent, Clone, Default)]
#[require(Velocity)]
struct Paddle;

const PADDLE_SHAPE: Rectangle = Rectangle::new(100., 10.);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);
const PADDLE_SPEED: f32 = 5.;
const PADDLE_Y: f32 = -300.;

impl Paddle {
    fn scene() -> impl Scene {
        let x = 0.;
        let y = PADDLE_Y;
        bsn! {
            Position(vec2(x,y))
            Collider(PADDLE_SHAPE)
            Mesh2d(asset_value(PADDLE_SHAPE))
            MeshMaterial2d<ColorMaterial>(asset_value(PADDLE_COLOR))
            DespawnOnExit<AppState>(AppState::InGame)
        }
    }
}

#[derive(Component, Clone, Default)]
struct Gutter;

const GUTTER_COLOR: Color = Color::srgb(0., 0., 1.);
const GUTTER_WIDTH: f32 = 20.;

fn gutter(x: f32, y: f32, shape: Rectangle) -> impl Scene {
    bsn! {
        Gutter
        Position(vec2(x, y))
        Collider(shape)
        Mesh2d(asset_value(shape))
        MeshMaterial2d<ColorMaterial>(asset_value(GUTTER_COLOR))
        DespawnOnExit<AppState>(AppState::InGame)
    }
}

#[derive(Component, Clone, Default)]
#[require(Position, Collider)]
struct Brick;

const BRICK_COLOR: Color = Color::srgb(1., 1., 1.);
const BRICK_SHAPE: Rectangle = Rectangle::new(60., 20.);

fn brick(x: f32, y: f32) -> impl Scene {
    bsn! {
        Brick
        Position(vec2(x, y))
        Collider(BRICK_SHAPE)
        Mesh2d(asset_value(BRICK_SHAPE))
        MeshMaterial2d<ColorMaterial>(asset_value(BRICK_COLOR))
        DespawnOnExit<AppState>(AppState::InGame)
    }
}

fn spawn_bricks(mut commands: Commands) {
    for line in 0..3 {
        for col in 0..10 {
            let x = (col * 100 - 500) as f32;
            let y = (line * 50 + 200) as f32;
            commands.spawn_scene(brick(x, y));
        }
    }
}

fn init_resources(mut commands: Commands) {
    commands.insert_resource(Lives(3));
    commands.insert_resource(Score(0));
}

fn check_out_of_lives(mut next_state: ResMut<NextState<GameState>>, lives: Res<Lives>) {
    if lives.0 == 0 {
        next_state.set(GameState::GameOver);
    }
}

fn update_lives_display(mut text: Single<&mut Text, With<LivesDisplay>>, lives: Res<Lives>) {
    info!("Lives updated: {}", lives.0);
    text.0 = format!("{} lives", lives.0);
}

fn update_score_display(mut text: Single<&mut Text, With<ScoreDisplay>>, score: Res<Score>) {
    info!("Score updated: {}", score.0);
    text.0 = format!("{} points", score.0);
}

fn set_win_state(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameOver);
}

fn handle_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_velocity: Single<&mut Velocity, With<Paddle>>,
) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        paddle_velocity.0.x = -PADDLE_SPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        paddle_velocity.0.x = PADDLE_SPEED;
    } else {
        paddle_velocity.0.x = 0.;
    }
}

fn launch_ball(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    ball: Single<Entity, (With<Ball>, Without<Velocity>)>,
) {
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ArrowUp) {
        commands.entity(*ball).insert(Velocity(BALL_BASE_VELOCITY));
    }
}

fn move_paddle(paddle: Single<(&mut Position, &Velocity), With<Paddle>>) {
    let (mut position, velocity) = paddle.into_inner();
    position.0 += velocity.0;
}

mod collision {
    use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    pub enum Collision {
        Left,
        Right,
        Top,
        Bottom,
    }

    // Returns `Some` if `ball` collides with `wall`. The returned `Collision` is the
    // side of `wall` that `ball` hit.
    pub fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
        if !ball.intersects(&wall) {
            return None;
        }

        let closest_point = wall.closest_point(ball.center());
        let offset = ball.center() - closest_point;

        let side = if offset.x.abs() > offset.y.abs() {
            if offset.x < 0. {
                Collision::Left
            } else {
                Collision::Right
            }
        } else if offset.y > 0. {
            Collision::Top
        } else {
            Collision::Bottom
        };

        Some(side)
    }
}

fn constrain_paddle_position(
    paddles: Single<(&mut Position, &Collider), (With<Paddle>, Without<Gutter>)>,
    gutters: Query<(&Position, &Collider), (With<Gutter>, Without<Paddle>)>,
) {
    let (mut paddle_position, paddle_collider) = paddles.into_inner();
    for (gutter_position, gutter_collider) in &gutters {
        let paddle_aabb = Aabb2d::new(paddle_position.0, paddle_collider.half_size());
        let gutter_aabb = Aabb2d::new(gutter_position.0, gutter_collider.half_size());

        if let Some(collision) = collision::collide_with_side(paddle_aabb, gutter_aabb) {
            match collision {
                collision::Collision::Left => {
                    paddle_position.0.x = gutter_position.0.x
                        - gutter_collider.half_size().x
                        - paddle_collider.half_size().x;
                }
                collision::Collision::Right => {
                    paddle_position.0.x = gutter_position.0.x
                        + gutter_collider.half_size().x
                        + paddle_collider.half_size().x;
                }
                _ => {}
            }
        }
    }
}

fn handle_lost_ball(
    mut commands: Commands,
    ball: Single<(Entity, &Position), With<Ball>>,
    mut lives: ResMut<Lives>,
) {
    let (ball_entity, ball_position) = ball.into_inner();
    if ball_position.0.y < PADDLE_Y - 100. {
        lives.0 -= 1;
        commands.entity(ball_entity).remove::<Velocity>();
    }
}

fn destroy_brick(event: On<BrickDestroyed>, mut commands: Commands, mut score: ResMut<Score>) {
    commands.entity(event.entity).despawn();
    score.0 += 10;
}

fn handle_collisions(
    mut commands: Commands,
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider, Has<Paddle>, Has<Brick>, Entity), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();
    let mut has_despawned = false;

    for (other_position, other_collider, is_paddle, is_brick, entity) in &other_things {
        let Some(collision) = collision::collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size()),
        ) else {
            continue;
        };
        if is_paddle {
            let paddle_pos = Vec2 {
                x: other_position.0.x,
                y: other_position.0.y + other_collider.half_size().y - other_collider.half_size().x,
            };
            let dir = (ball_position.0 - paddle_pos).normalize();
            ball_velocity.0 = dir * BALL_SPEED;
            continue;
        }
        if is_brick && has_despawned {
            continue;
        }
        match collision {
            collision::Collision::Left | collision::Collision::Right => {
                ball_velocity.0.x *= -1.;
            }
            collision::Collision::Top | collision::Collision::Bottom => {
                ball_velocity.0.y *= -1.;
            }
        }
        if is_brick {
            commands.trigger(BrickDestroyed { entity });
            has_despawned = true;
        }
    }
}

fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0 * BALL_SPEED;
}

fn move_locked_ball(
    mut ball: Single<&mut Position, (With<Ball>, Without<Velocity>)>,
    paddle: Single<&Position, (With<Paddle>, Without<Ball>)>,
) {
    ball.0 = paddle.0 + vec2(0., 20.);
}

fn spawn_entities(mut commands: Commands, window: Single<&Window>) {
    let half_width = window.resolution.width() / 2.;
    let half_height = window.resolution.height() / 2.;
    let shape_v = Rectangle::new(GUTTER_WIDTH, window.resolution.height());
    let shape_h = Rectangle::new(window.resolution.width(), GUTTER_WIDTH);

    commands.spawn_scene_list(bsn_list! [
        @Ball,
        @Paddle,
        gutter(half_width, 0., shape_v),
        gutter(-half_width, 0., shape_v),
        gutter(0., half_height, shape_h),
    ]);
}
