use bevy::prelude::*;
use avian3d::prelude::*;
use crate::components::Player;

// Combat Components
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            current: max_health,
            max: max_health,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn take_damage(&mut self, damage: f32) -> f32 {
        let damage_taken = damage.min(self.current);
        self.current -= damage_taken;
        damage_taken
    }

    pub fn heal(&mut self, amount: f32) -> f32 {
        let heal_amount = amount.min(self.max - self.current);
        self.current += heal_amount;
        heal_amount
    }

    pub fn health_percentage(&self) -> f32 {
        if self.max <= 0.0 { 0.0 } else { self.current / self.max }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyType {
    ForestGuardian,
    RockElemental,
    WildBoar,
}

impl EnemyType {
    pub fn name(self) -> &'static str {
        match self {
            EnemyType::ForestGuardian => "Forest Guardian",
            EnemyType::RockElemental => "Rock Elemental", 
            EnemyType::WildBoar => "Wild Boar",
        }
    }

    pub fn max_health(self) -> f32 {
        match self {
            EnemyType::ForestGuardian => 100.0,
            EnemyType::RockElemental => 150.0,
            EnemyType::WildBoar => 80.0,
        }
    }

    pub fn damage(self) -> f32 {
        match self {
            EnemyType::ForestGuardian => 15.0,
            EnemyType::RockElemental => 25.0,
            EnemyType::WildBoar => 20.0,
        }
    }

    pub fn experience_reward(self) -> u64 {
        match self {
            EnemyType::ForestGuardian => 50,
            EnemyType::RockElemental => 75,
            EnemyType::WildBoar => 40,
        }
    }

    pub fn color(self) -> Color {
        match self {
            EnemyType::ForestGuardian => Color::srgb(0.2, 0.8, 0.3),
            EnemyType::RockElemental => Color::srgb(0.6, 0.6, 0.6),
            EnemyType::WildBoar => Color::srgb(0.6, 0.4, 0.2),
        }
    }
}

#[derive(Component)]
pub struct CombatTarget;

#[derive(Component)]
pub struct AutoAttack {
    pub timer: Timer,
    pub damage: f32,
}

impl AutoAttack {
    pub fn new(attack_speed: f32, damage: f32) -> Self {
        Self {
            timer: Timer::from_seconds(attack_speed, TimerMode::Repeating),
            damage,
        }
    }
}

// Combat Resources
#[derive(Resource, Default)]
pub struct CombatState {
    pub player_target: Option<Entity>,
    pub in_combat: bool,
}

#[derive(Resource)]
pub struct CombatConfig {
    pub auto_attack_range: f32,
    pub target_selection_range: f32,
    pub base_player_damage: f32,
}

impl Default for CombatConfig {
    fn default() -> Self {
        Self {
            auto_attack_range: 3.0,
            target_selection_range: 15.0,
            base_player_damage: 25.0,
        }
    }
}

// System to spawn demo enemies
pub fn spawn_demo_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawned: Local<bool>,
) {
    if *spawned {
        return;
    }

    info!("🐺 Spawning demo enemies");

    let enemy_positions = [
        (Vec3::new(20.0, 5.0, 30.0), EnemyType::ForestGuardian),
        (Vec3::new(-30.0, 8.0, 40.0), EnemyType::RockElemental),
        (Vec3::new(50.0, 3.0, -20.0), EnemyType::WildBoar),
        (Vec3::new(-15.0, 6.0, -35.0), EnemyType::ForestGuardian),
        (Vec3::new(0.0, 10.0, 60.0), EnemyType::RockElemental),
    ];

    for (position, enemy_type) in enemy_positions {
        let mesh = meshes.add(Cuboid::new(2.0, 3.0, 2.0));
        let material = materials.add(StandardMaterial {
            base_color: enemy_type.color(),
            ..default()
        });

        let _enemy_entity = commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position),
            RigidBody::Kinematic,
            Collider::cuboid(1.0, 1.5, 1.0),
            Enemy { enemy_type },
            Health::new(enemy_type.max_health()),
        )).id();

        info!("Spawned {} at {:?}", enemy_type.name(), position);
    }

    *spawned = true;
}

// System to handle target selection (Tab key cycling)
pub fn handle_target_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut combat_state: ResMut<CombatState>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Enemy, &Health), Without<Player>>,
    combat_config: Res<CombatConfig>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        if let Ok(player_transform) = player_query.single() {
            // Get all enemies within targeting range
            let mut targets: Vec<_> = enemy_query
                .iter()
                .filter(|(_, enemy_transform, _, health)| {
                    health.is_alive() && 
                    player_transform.translation.distance(enemy_transform.translation) <= combat_config.target_selection_range
                })
                .collect();

            // Sort by distance
            targets.sort_by(|a, b| {
                let dist_a = player_transform.translation.distance(a.1.translation);
                let dist_b = player_transform.translation.distance(b.1.translation);
                dist_a.partial_cmp(&dist_b).unwrap()
            });

            if !targets.is_empty() {
                // If no current target or current target is not in range, select closest
                if let Some(current_target) = combat_state.player_target {
                    if let Some(current_index) = targets.iter().position(|(entity, _, _, _)| *entity == current_target) {
                        // Cycle to next target
                        let next_index = (current_index + 1) % targets.len();
                        combat_state.player_target = Some(targets[next_index].0);
                        info!("🎯 Target switched to {}", targets[next_index].2.enemy_type.name());
                    } else {
                        // Current target not in range, select closest
                        combat_state.player_target = Some(targets[0].0);
                        info!("🎯 Target selected: {}", targets[0].2.enemy_type.name());
                    }
                } else {
                    // No current target, select closest
                    combat_state.player_target = Some(targets[0].0);
                    info!("🎯 Target selected: {}", targets[0].2.enemy_type.name());
                }
                
                combat_state.in_combat = true;
            } else {
                info!("No valid targets in range");
                combat_state.player_target = None;
                combat_state.in_combat = false;
            }
        }
    }

    // Clear target if it's dead or out of range
    if let Some(target_entity) = combat_state.player_target {
        if let Ok(player_transform) = player_query.single() {
            if let Ok((_, target_transform, _, target_health)) = enemy_query.get(target_entity) {
                let distance = player_transform.translation.distance(target_transform.translation);
                if !target_health.is_alive() || distance > combat_config.target_selection_range * 1.5 {
                    combat_state.player_target = None;
                    combat_state.in_combat = false;
                    info!("Target lost - too far or dead");
                }
            } else {
                combat_state.player_target = None;
                combat_state.in_combat = false;
            }
        }
    }
}

// System to handle player auto-attack
pub fn handle_player_auto_attack(
    time: Res<Time>,
    mut combat_state: ResMut<CombatState>,
    combat_config: Res<CombatConfig>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Health, &Enemy), Without<Player>>,
    mut player_auto_attack: Local<Option<AutoAttack>>,
) {
    // Initialize auto-attack timer if needed
    if player_auto_attack.is_none() {
        *player_auto_attack = Some(AutoAttack::new(1.8, combat_config.base_player_damage));
    }

    if let Some(ref mut auto_attack) = player_auto_attack.as_mut() {
        auto_attack.timer.tick(time.delta());

        if let (Some(target_entity), Ok(player_transform)) = (combat_state.player_target, player_query.single()) {
            if let Ok((target_transform, mut target_health, enemy)) = enemy_query.get_mut(target_entity) {
                let distance = player_transform.translation.distance(target_transform.translation);
                
                if distance <= combat_config.auto_attack_range && auto_attack.timer.just_finished() {
                    // Deal damage
                    let damage_dealt = target_health.take_damage(auto_attack.damage);
                    
                    info!("⚔️ Player attacks {} for {:.1} damage! ({:.1}/{:.1} HP remaining)",
                          enemy.enemy_type.name(), damage_dealt, target_health.current, target_health.max);

                    if !target_health.is_alive() {
                        info!("💀 {} defeated!", enemy.enemy_type.name());
                        
                        // Award experience - this would be better done through an event system
                        let experience_reward = enemy.enemy_type.experience_reward();
                        info!("📈 Gained {} experience from combat!", experience_reward);
                        
                        // Clear target
                        combat_state.player_target = None;
                        combat_state.in_combat = false;
                    }
                }
            }
        }
    }
}

// System to remove dead enemies
pub fn cleanup_dead_enemies(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Health, &Enemy)>,
) {
    for (entity, health, enemy) in &enemy_query {
        if !health.is_alive() {
            commands.entity(entity).despawn();
            info!("🗑️ Removed dead {} from world", enemy.enemy_type.name());
        }
    }
}

// System to display target health above enemies
pub fn display_target_health(
    combat_state: Res<CombatState>,
    mut gizmos: Gizmos,
    enemy_query: Query<(&Transform, &Health, &Enemy)>,
) {
    if let Some(target_entity) = combat_state.player_target {
        if let Ok((transform, health, _enemy)) = enemy_query.get(target_entity) {
            let position = transform.translation + Vec3::Y * 4.0;
            
            // Draw health bar background (red)
            gizmos.cuboid(
                Transform::from_translation(position)
                    .with_scale(Vec3::new(3.0, 0.3, 0.1)),
                Color::srgb(0.8, 0.2, 0.2),
            );
            
            // Draw health bar fill (green)
            let health_width = 3.0 * health.health_percentage();
            let health_offset = Vec3::X * (3.0 - health_width) * -0.5;
            gizmos.cuboid(
                Transform::from_translation(position + health_offset)
                    .with_scale(Vec3::new(health_width, 0.25, 0.15)),
                Color::srgb(0.2, 0.8, 0.2),
            );
            
            // Draw target indicator
            gizmos.cuboid(
                Transform::from_translation(transform.translation + Vec3::Y * 0.1)
                    .with_scale(Vec3::new(3.0, 0.1, 3.0)),
                Color::srgb(1.0, 1.0, 0.0), // Yellow target indicator
            );
        }
    }
}

// System to setup combat resources
pub fn setup_combat_system(mut commands: Commands) {
    commands.insert_resource(CombatState::default());
    commands.insert_resource(CombatConfig::default());
    info!("⚔️ Combat system initialized");
}