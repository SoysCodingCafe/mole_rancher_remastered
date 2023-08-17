// Import Bevy game engine essentials
use bevy::{prelude::*, window::WindowResized, render::camera::Viewport, input::mouse::{MouseMotion, MouseWheel}};
// Import components, resources, and events
use crate::components::*;

// Plugin for handling camera movement and resizing
// the viewport for the reactor camera
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
		.add_systems( Update,(
			resize_reactor_camera_viewport,
			pan_zoom_reactor_camera,
		))
		.add_systems( OnEnter(GameState::Reactor),(
			reset_reactor_camera,
		))
		;
	}
}

// Resets the camera view back to the center of the screen
// when re-entering the reactor
fn reset_reactor_camera(
	mut reactor_camera_query: Query<(&mut OrthographicProjection, &mut Transform, With<ReactorCamera>)>,
) {
	let (mut ortho_proj, mut transform, _) = reactor_camera_query.single_mut();
	ortho_proj.scale = 4.0;
	transform.translation.x = 0.0;
	transform.translation.y = 0.0;
}

// Detects window resize events and scales the viewport to match
// the current size of the window
fn resize_reactor_camera_viewport(
    window_query: Query<&Window>,
	ortho_size: Res<OrthoSize>,
    mut ev_r_resize: EventReader<WindowResized>,
    mut reactor_camera_query: Query<&mut Camera, With<ReactorCamera>>,
) {
	let size = Dimensions {
		width: REACTOR_VIEWPORT_WIDTH,
		height: REACTOR_VIEWPORT_HEIGHT,
	};
	let location = Vec2::new(
		REACTOR_VIEWPORT_X,
		REACTOR_VIEWPORT_Y	
	);
	let window = window_query.single();
    for _ in ev_r_resize.iter() {
		let mut camera = reactor_camera_query.single_mut();
		camera.viewport = Some(Viewport {
			physical_position: UVec2::new(
				(window.width() / ortho_size.width * location.x) as u32,
				(window.height()  / ortho_size.height * location.y) as u32,
			),
			physical_size: UVec2::new(
				(window.width() / ortho_size.width * size.width) as u32,
				(window.height()  / ortho_size.height * size.height) as u32,
			),
			..default()
		});
    }
}

// Allows the user to pan and zoom the camera when in 
// the reactor viewport
fn pan_zoom_reactor_camera(
	ortho_size: Res<OrthoSize>,
	mouse: Res<Input<MouseButton>>,
	window_query: Query<&Window>,
	tracked_query: Query<(Entity, &Transform, With<SelectedMolecule>)>,
	mut reactor_camera_query: Query<(&mut Transform, &mut OrthographicProjection, With<ReactorCamera>, Without<SelectedMolecule>)>,
	mut commands: Commands,
    mut ev_r_motion: EventReader<MouseMotion>,
    mut ev_r_scroll: EventReader<MouseWheel>,	
) {
    let mut pan = Vec2::ZERO;
    let mut scroll = 0.0;

	// Get the current window, and the cursor position scaled 
	// to the window size
	let w = window_query.single();
	let mut offset = Vec2::ZERO;
	if let Some(p) = w.cursor_position() {
		let p = Vec2::new(
			ortho_size.width * (p.x / w.width() - 0.5), 
			-ortho_size.height * (p.y / w.height() - 0.5)
		);
		if (p.x - REACTOR_VIEWPORT_CENTER.x).abs() <= REACTOR_VIEWPORT_WIDTH / 2.0 && 
		(p.y - REACTOR_VIEWPORT_CENTER.y).abs() <= REACTOR_VIEWPORT_HEIGHT / 2.0 {
			if mouse.pressed(MouseButton::Right) {
				for ev in ev_r_motion.iter() {
					pan += ev.delta;
				}
			}
			for ev in ev_r_scroll.iter() {
				scroll += ev.y;
				if (p - REACTOR_VIEWPORT_CENTER).length() > ZOOM_DEAD_ZONE_RADIUS {
					offset = (p - REACTOR_VIEWPORT_CENTER).normalize();
				}
			}
		}
	}

	let scale_limits = (MAX_ZOOM, MIN_ZOOM);
    for (mut transform, mut ortho_projection, _, _) in reactor_camera_query.iter_mut() {
		if scroll.abs() > 0.0 {
			ortho_projection.scale = ((ortho_projection.scale.ln() - scroll/ZOOM_SPEED).exp()).clamp(scale_limits.0, scale_limits.1);
			if ortho_projection.scale != scale_limits.0 && ortho_projection.scale != scale_limits.1 {
				transform.translation.x = transform.translation.x + offset.x * ortho_projection.scale * ZOOM_TRANSLATION_SPEED * scroll.signum();
				transform.translation.y = transform.translation.y + offset.y * ortho_projection.scale * ZOOM_TRANSLATION_SPEED * scroll.signum();
			}
		}
		if pan.length_squared() > 0.0 {
			transform.translation.x = transform.translation.x - pan.x * ortho_projection.scale;
			transform.translation.y = transform.translation.y + pan.y * ortho_projection.scale;
			for (tracked_entity, _, _) in tracked_query.iter() {
				commands.entity(tracked_entity).remove::<SelectedMolecule>();
			}
		} else {
			for (_, tracked_transform, _) in tracked_query.iter() {
				transform.translation.x = (transform.translation.x * 3.0 + tracked_transform.translation.x) / 4.0;
				transform.translation.y = (transform.translation.y * 3.0 + tracked_transform.translation.y) / 4.0;
			};
		}
		transform.translation.x = transform.translation.x.clamp(
			(-scale_limits.1 * ortho_size.width + ortho_projection.scale * ortho_size.width) / 2.0,
			(scale_limits.1 * ortho_size.width - ortho_projection.scale * ortho_size.width) / 2.0,
		);
		transform.translation.y = transform.translation.y.clamp(
			(-scale_limits.1 * ortho_size.height + ortho_projection.scale * ortho_size.height) / 2.0,
			(scale_limits.1 * ortho_size.height - ortho_projection.scale * ortho_size.height) / 2.0,
		);
    }
    ev_r_motion.clear();
}