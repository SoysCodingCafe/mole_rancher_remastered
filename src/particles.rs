// Import Bevy game engine essentials
use bevy::{prelude::*, render::view::RenderLayers};
// Import components, resources, and events
use crate::components::*;

// Plugin for handling particle trail emitters
// and fading the particles over time
pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_systems(Update, (
				spawn_particles,
				fade_particles,
			))
		;
	}
}

fn spawn_particles(
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut particle_trail_query: Query<(&Transform, &MoleculeInfo, &mut ParticleTrail)>,
) {
	for (transform, m_info, mut trail) in particle_trail_query.iter_mut() {
		trail.spawn_timer.tick(time.delta());
		if trail.spawn_timer.just_finished() {
			commands
				.spawn((SpriteBundle {
					transform: Transform::from_xyz(
						transform.translation.x + (rand::random::<f32>() - 0.5) * 8.0, 
						transform.translation.y + (rand::random::<f32>() - 0.5) * 8.0, 
						transform.translation.z - 1.0),
					texture: asset_server.load("sprites/ui/circle.png"),
					sprite: Sprite {
						//color: Color::rgb(rand::random(), rand::random(), rand::random()),
						custom_size: Some(Vec2::new(m_info.radius * 2.0, m_info.radius * 2.0)),
						..Default::default()
					},
					..Default::default()
				},
				Particle{duration: Timer::from_seconds(trail.duration, TimerMode::Once)},
				RenderLayers::layer(1),
				DespawnOnExitGameState,
				Name::new("Particle")
			));
		}
	}
}

fn fade_particles(
	mut commands: Commands,
	mut particle_query: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
	time: Res<Time>,
) {
	for (entity, mut transform, mut sprite, mut particle) in particle_query.iter_mut() {
		particle.duration.tick(time.delta());
		transform.scale.x = (1.0 - particle.duration.percent()).clamp(0.1, 1.0);
		transform.scale.y = (1.0 - particle.duration.percent()).clamp(0.1, 1.0);
		sprite.color.set_a((1.0 - particle.duration.percent()).clamp(0.1, 1.0));
		if particle.duration.just_finished() {
			commands.entity(entity).despawn_recursive();
		}
	}
}