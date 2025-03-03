pub mod prelude;

pub mod user;

pub mod color;
pub mod control_data;
pub mod control_frame;
pub mod dancer;
pub mod editing_control_frame;
pub mod editing_led_effect;
pub mod editing_position_frame;
pub mod effect_list_data;
pub mod led_bulb;
pub mod led_effect;
pub mod led_effect_state;
pub mod logger;
pub mod model;
pub mod part;
pub mod position_data;
pub mod position_frame;
pub mod revision;
pub mod sea_orm_active_enums;

seaography::register_entity_modules!([
    color,
    control_data,
    control_frame,
    dancer,
    editing_control_frame,
    editing_led_effect,
    editing_position_frame,
    effect_list_data,
    led_bulb,
    led_effect,
    led_effect_state,
    logger,
    model,
    part,
    position_data,
    position_frame,
    revision,
]);
seaography::register_active_enums!([sea_orm_active_enums::Type]);
