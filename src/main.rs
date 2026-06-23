//! This example shows off the various Bevy Feathers widgets.

#![allow(clippy::too_many_arguments)]

use bevy::{
    feathers::{
        controls::*,
        cursor::{EntityCursor, OverrideCursor},
        dark_theme::create_dark_theme,
        rounded_corners::RoundedCorners,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens, FeathersPlugins,
    },
    input_focus::{tab_navigation::TabGroup, AutoFocus },
    prelude::*,
    ui::{Checked, InteractionDisabled, },
    ui_widgets::{
        checkbox_self_update, radio_self_update, slider_self_update,
        Activate, ActivateOnPress, RadioGroup, SliderPrecision, SliderStep,
        ValueChange,
    },
    window::SystemCursorIcon,
};

#[derive(Component, Clone, Copy, Default)]
struct DemoDisabledButton;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FeathersPlugins))
        .insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, scene.spawn())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list![Camera2d, demo_root()]
}

fn demo_root() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(8),
        }
        TabGroup
        ThemeBackgroundColor(tokens::WINDOW_BG)
        Children[
            demo_column_1(),
            // demo_column_2(),
        ]
    }
}

fn demo_column_1() -> impl Scene {
    bsn! {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::Start,
            padding: px(8),
            row_gap: px(8),
            width: percent(100),
            // min_width: px(200),
        }
        Children [
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: px(8),
                }
                Children [
                    (
                        @FeathersButton {
                            @caption: bsn! { Text("Normal") ThemedText }
                        }
                        Node {
                            flex_grow: 1.0,
                        }
                        AccessibleLabel("Normal")
                        on(|_activate: On<Activate>| {
                            info!("Normal button clicked!");
                        })
                        AutoFocus
                    ),
                    (
                        @FeathersButton {
                            @caption: bsn! { Text("Disabled") ThemedText },
                        }
                        Node {
                            flex_grow: 1.0,
                        }
                        AccessibleLabel("Disabled")
                        InteractionDisabled
                        DemoDisabledButton
                        on(|_activate: On<Activate>| {
                            info!("Disabled button clicked!");
                        })
                    ),
                    (
                        @FeathersButton {
                            @caption: bsn! { Text("Primary") ThemedText },
                            @variant: ButtonVariant::Primary,
                        }
                        AccessibleLabel("Primary")
                        Node {
                            flex_grow: 1.0,
                        }
                        on(|_activate: On<Activate>| {
                            info!("Primary button clicked!");
                        })
                    ),
                    (
                        @FeathersMenu
                        Children [
                            (
                                @FeathersMenuButton {
                                    @caption: bsn! { Text("Menu") ThemedText }
                                }
                                AccessibleLabel("Menu Example")
                                Node {
                                    flex_grow: 1.0,
                                }
                            ),
                            (
                                @FeathersMenuPopup
                                Children [
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("MenuItem 1") ThemedText }
                                        }
                                        on(|_: On<Activate>| {
                                            info!("Menu item 1 clicked!");
                                        })
                                    ),
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("MenuItem 2") ThemedText }
                                        }
                                        on(|_: On<Activate>| {
                                            info!("Menu item 2 clicked!");
                                        })
                                    ),
                                    @FeathersMenuDivider,
                                    (
                                        @FeathersMenuItem {
                                            @caption: bsn! { Text("MenuItem 3") ThemedText }
                                        }
                                        on(|_: On<Activate>| {
                                            info!("Menu item 3 clicked!");
                                        })
                                    )
                                ]
                            )
                        ]
                    )
                ]
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: px(1),
                }
                Children [
                    (
                        @FeathersButton {
                            @caption: bsn! { Text("Left") ThemedText },
                            @corners: RoundedCorners::Left,
                        }
                        Node {
                            flex_grow: 1.0,
                        }
                        AccessibleLabel("Left")
                        on(|_activate: On<Activate>| {
                            info!("Left button clicked!");
                        })
                    ),
                    (
                        @FeathersButton {
                            @caption: bsn! { Text("Center") ThemedText },
                            @corners: RoundedCorners::None,
                        }
                        Node {
                            flex_grow: 1.0,
                        }
                        AccessibleLabel("Center")
                        on(|_activate: On<Activate>| {
                            info!("Center button clicked!");
                        })
                    ),
                    (
                        @FeathersButton {
                            @caption: bsn! { Text("Right") ThemedText },
                            @variant: ButtonVariant::Primary,
                            @corners: RoundedCorners::Right,
                        }
                        Node {
                            flex_grow: 1.0,
                        }
                        AccessibleLabel("Right")
                        on(|_activate: On<Activate>| {
                            info!("Right button clicked!");
                        })
                    ),
                ]
            ),
            (
                @FeathersButton
                on(|_activate: On<Activate>, mut ovr: ResMut<OverrideCursor>| {
                    ovr.0 = if ovr.0.is_some() {
                        None
                    } else {
                        Some(EntityCursor::System(SystemCursorIcon::Wait))
                    };
                    info!("Override cursor button clicked!");
                })
                Children [ (Text("Toggle override") ThemedText) ]
            ),
            (
                @FeathersCheckbox {
                    @caption: bsn! { Text("Checkbox") ThemedText }
                }
                Checked
                AccessibleLabel("Checkbox Example")
                on(
                    |change: On<ValueChange<bool>>,
                        query: Query<Entity, With<DemoDisabledButton>>,
                        mut commands: Commands| {
                        info!("Checkbox clicked!");
                        let mut button = commands.entity(query.single().unwrap());
                        if change.value {
                            button.insert(InteractionDisabled);
                        } else {
                            button.remove::<InteractionDisabled>();
                        }
                        let mut checkbox = commands.entity(change.source);
                        if change.value {
                            checkbox.insert(Checked);
                        } else {
                            checkbox.remove::<Checked>();
                        }
                    }
                )
            ),
            (
                @FeathersCheckbox {
                    @caption: bsn! { Text("Fast Click Checkbox") ThemedText }
                }
                ActivateOnPress
                AccessibleLabel("Fast Click Checkbox Example")
                on(
                    |change: On<ValueChange<bool>>,
                     mut commands: Commands| {
                        info!("Checkbox clicked!");
                        let mut checkbox = commands.entity(change.source);
                        if change.value {
                            checkbox.insert(Checked);
                        } else {
                            checkbox.remove::<Checked>();
                        }
                    }
                )
            ),
            (
                @FeathersCheckbox {
                    @caption: bsn! { Text("Disabled") ThemedText },
                }
                InteractionDisabled
                AccessibleLabel("Disabled Checkbox Example")
                on(|_change: On<ValueChange<bool>>| {
                    warn!("Disabled checkbox clicked!");
                })
            ),
            (
                @FeathersCheckbox {
                    @caption: bsn! { Text("Checked+Disabled") ThemedText }
                }
                InteractionDisabled
                Checked
                AccessibleLabel("Disabled and Checked Checkbox Example")
                on(|_change: On<ValueChange<bool>>| {
                    warn!("Disabled checkbox clicked!");
                })
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: px(8),
                }
                Children [
                    (
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            row_gap: px(4),
                        }
                        RadioGroup
                        on(radio_self_update)
                        Children [
                            (
                                @FeathersRadio {
                                    @caption: bsn! { Text("One") ThemedText }
                                }
                                Checked
                            ),
                            @FeathersRadio {
                                @caption: bsn! { Text("Two") ThemedText }
                            },
                            (
                                @FeathersRadio {
                                    @caption: bsn! { Text("Fast Click") ThemedText }
                                }
                                ActivateOnPress
                            ),
                            (
                                @FeathersRadio {
                                    @caption: bsn! { Text("Disabled") ThemedText }
                                }
                                InteractionDisabled
                            ),
                        ]
                    )
                ]
            ),
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    column_gap: px(8),
                }
                Children [
                    (@FeathersToggleSwitch on(checkbox_self_update)),
                    (@FeathersToggleSwitch ActivateOnPress on(checkbox_self_update)),
                    (@FeathersToggleSwitch InteractionDisabled on(checkbox_self_update)),
                    (@FeathersToggleSwitch InteractionDisabled Checked on(checkbox_self_update)),
                    (@FeathersDisclosureToggle on(checkbox_self_update)),
                ]
            ),
            (
                @FeathersSlider {
                    @max: 100.0,
                    @value: 20.0,
                }
                SliderStep(10.)
                SliderPrecision(2)
                on(slider_self_update)
            ),
        ]
    }
}
