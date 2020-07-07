use crate::core::{msvc::vector::StdVector, riot::r3d::vector3::R3dVector3};

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraLogic {
    fps_camera: RiotFpsCamera,
    focus_client: RiotFocusCameraClient,
    camera_behavior_data: RiotCameraBehaviorData,
    attached_effect: StdVector<u32>, // std::__1::vector<gobjid_t,std::__1::allocator<gobjid_t> >
    move_to_point_camera_client: RiotMoveToPointCameraClient,
    follow_spline_camera_client: RiotFollowSplineCameraClient,
    attributes: RiotCameraLogicAttributes,
    camera_shake: RiotCameraLogicShake,
    circular_restiction: RiotCameraLogicCircularRestriction,
    events: [u8; 24], // HudCameraLogic::EventList
    smoothing: RiotCameraLogicSmoothing,
    zoom: RiotCameraLogicZoom,
    mode: RiotCameraLogicMode,
    camera_move_on_minimap: bool,
    camera_frozen: bool,
    snapping_camera_to_player: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotFpsCamera {
    view_angle_velocity_magnitude: f32,
    acceleration_magnitude: f32,
    position: R3dVector3,
    acceleration: R3dVector3,
    view_angle_velocity: R3dVector3,
    view_origin: R3dVector3,
    view_angle: R3dVector3,
    previous_attached_object_position: R3dVector3,
    view_direction: R3dVector3,
    view_right: R3dVector3,
    attached_object_id: u32,
    last_mouse_x: i32,
    last_mouse_y: i32,
    fps_mouse_dowm: bool,
    contrained_movement_xz: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotFocusCameraClient {
    yaw: f32,
    pitch: f32,
    x_offset: f32,
    y_offset: f32,
    zoom: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraBehaviorData {
    delay_time: f32,
    is_active: bool,
    position: R3dVector3,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotMoveToPointCameraClient {
    start_position: R3dVector3,
    target_position: R3dVector3,
    movement_direction: R3dVector3,
    total_movement_distance: f32,
    camera_velocity: f32,
    distance_travelled: f32,
    has_camera_arrived: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotFollowSplineCameraClient {
    has_camera_arrived: bool,
    points: StdVector<RiotFollowSplineCameraClientPoint>, // std::__1::vector<FollowSplineCameraClient::SplineCameraPoint *,std::__1::allocator<FollowSplineCameraClient::SplineCameraPoint *> >
    total_movement_distance: f32,
    distance_travelled: f32,
    start_time: f32,
    finish_time: f32,
    camera_interpolation_mode: RiotFollowSplineCameraClientInerpolationMode,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RiotFollowSplineCameraClientPoint {
    point: R3dVector3,
    progress: f32,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum RiotFollowSplineCameraClientInerpolationMode {
    Linear = 0,
    SmoothStop = 1,
    CatmullSlow = 2,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraLogicAttributes {
    world_position: R3dVector3,
    camera_freeze_point: R3dVector3,
    current_velocity: f32,
    current_pitch: f32,
    current_yaw: f32,
    start_yaw: f32,
    start_pitch: f32,
    current_fov: f32,
    last_click_map_time: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraLogicShake {
    position_offset: R3dVector3,
    duration: f32,
    start_time: f32,
    power: f32,
    next_shake_time: f32,
    shake_count_within_duration: f32,
    is_active: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraLogicCircularRestriction {
    center: R3dVector3,
    radius: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraLogicSmoothing {
    scroll_direction: R3dVector3,
    acceleration_time: f32,
    acceleration_time_keyboard: f32,
    deceleration_time_keyboard: f32,
    acceleration_time_mouse: f32,
    deceleration_time_mouse: f32,
    acceleration_state: RiotCameraLogicAccelerationState,
}

#[repr(u32)]
#[derive(Debug)]
pub enum RiotCameraLogicAccelerationState {
    None = 0,
    MouseAcceleration = 1,
    KeyboardAcceleration = 2,
    MouseDeceleration = 3,
    KeyboardDeceleration = 4,
}

#[repr(C)]
#[derive(Debug)]
pub struct RiotCameraLogicZoom {
    zoom_ease_time: f32,
    zoom_min_speed: f32,
    scale: f32,
    velocity: f32,
    current: f32,
    desired: f32,
}

#[repr(u32)]
#[derive(Debug)]
pub enum RiotCameraLogicMode {
    Topdown = 0,
    FPS = 1,
    TPS = 2,
    Focus = 3,
}