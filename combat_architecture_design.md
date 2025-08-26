# Combat System Architecture Design

## Component-Based Combat Architecture

This document outlines the technical implementation architecture for Eryndor's combat system using Bevy's ECS pattern. The design emphasizes modularity, performance, and extensibility.

## Core Combat Components

### CombatStats Component

```rust
#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub max_health: f32,
    pub current_health: f32,
    pub max_mana: f32,
    pub current_mana: f32,
    pub max_stamina: f32,
    pub current_stamina: f32,
    pub max_focus: f32,
    pub current_focus: f32,
    
    // Damage modifiers by type
    pub damage_modifiers: HashMap<DamageType, f32>,
    pub resistances: HashMap<DamageType, f32>,
    
    // Combat state
    pub is_in_combat: bool,
    pub combat_timer: f32,
    pub last_damage_time: f32,
    
    // Regeneration rates
    pub health_regen_rate: f32,
    pub mana_regen_rate: f32,
    pub stamina_regen_rate: f32,
    pub focus_regen_rate: f32,
}

impl CombatStats {
    pub fn new(level: u32, build_type: BuildType) -> Self {
        // Constructor logic based on character level and build
    }
    
    pub fn take_damage(&mut self, amount: f32, damage_type: DamageType) -> f32 {
        // Apply resistances and deal damage
        let resistance = self.resistances.get(&damage_type).unwrap_or(&0.0);
        let actual_damage = amount * (1.0 - resistance);
        self.current_health = (self.current_health - actual_damage).max(0.0);
        self.enter_combat();
        actual_damage
    }
    
    pub fn consume_resource(&mut self, resource_type: ResourceType, amount: f32) -> bool {
        // Attempt to consume the specified resource
        match resource_type {
            ResourceType::Mana => {
                if self.current_mana >= amount {
                    self.current_mana -= amount;
                    true
                } else { false }
            },
            // ... other resource types
        }
    }
    
    pub fn enter_combat(&mut self) {
        self.is_in_combat = true;
        self.combat_timer = COMBAT_DURATION;
        self.last_damage_time = 0.0;
    }
}
```

### TargetingSystem Component

```rust
#[derive(Component, Debug)]
pub struct TargetingSystem {
    pub current_target: Option<Entity>,
    pub target_type: TargetType,
    pub max_targeting_range: f32,
    pub targeting_mode: TargetingMode,
    pub last_target_update: f32,
    pub hostile_entities: Vec<Entity>,
    pub friendly_entities: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub enum TargetType {
    Enemy,
    Ally,
    Self_,
    Ground(Vec3),
    Area(Vec3, f32), // Position and radius
}

#[derive(Debug, Clone)]
pub enum TargetingMode {
    Tab,        // Tab through nearby enemies
    Click,      // Click to target
    Nearest,    // Auto-target nearest enemy
    Lowest,     // Auto-target lowest health enemy
}

impl TargetingSystem {
    pub fn set_target(&mut self, target: Option<Entity>, target_type: TargetType) {
        self.current_target = target;
        self.target_type = target_type;
        self.last_target_update = 0.0;
    }
    
    pub fn clear_target(&mut self) {
        self.current_target = None;
    }
    
    pub fn is_valid_target(&self, entity: Entity, max_range: f32, player_pos: Vec3, target_pos: Vec3) -> bool {
        // Validate target based on range, line of sight, and hostility
        let distance = player_pos.distance(target_pos);
        distance <= max_range && self.has_line_of_sight(player_pos, target_pos)
    }
    
    pub fn has_line_of_sight(&self, from: Vec3, to: Vec3) -> bool {
        // Ray casting for line of sight validation
        true // Placeholder - implement with physics ray casting
    }
}
```

### AutoAttackSystem Component

```rust
#[derive(Component, Debug)]
pub struct AutoAttackSystem {
    pub is_enabled: bool,
    pub attack_interval: f32,
    pub time_since_last_attack: f32,
    pub weapon_range: f32,
    pub base_damage: f32,
    pub damage_type: DamageType,
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub animation_name: String,
    pub hit_timing: f32, // When in the animation to apply damage
}

impl AutoAttackSystem {
    pub fn new_from_weapon(weapon: &WeaponStats) -> Self {
        Self {
            is_enabled: true,
            attack_interval: weapon.attack_speed,
            time_since_last_attack: 0.0,
            weapon_range: weapon.range,
            base_damage: weapon.damage,
            damage_type: weapon.damage_type,
            critical_chance: weapon.critical_chance,
            critical_multiplier: weapon.critical_multiplier,
            animation_name: weapon.attack_animation.clone(),
            hit_timing: weapon.hit_timing,
        }
    }
    
    pub fn can_attack(&self) -> bool {
        self.is_enabled && self.time_since_last_attack >= self.attack_interval
    }
    
    pub fn reset_attack_timer(&mut self) {
        self.time_since_last_attack = 0.0;
    }
    
    pub fn calculate_damage(&self, attacker_stats: &CombatStats) -> (f32, bool) {
        let base = self.base_damage * (1.0 + attacker_stats.damage_modifiers.get(&self.damage_type).unwrap_or(&0.0));
        let is_critical = fastrand::f32() < self.critical_chance;
        let final_damage = if is_critical { base * self.critical_multiplier } else { base };
        (final_damage, is_critical)
    }
}
```

### AbilitySystem Component

```rust
#[derive(Component, Debug)]
pub struct AbilitySystem {
    pub hotbar_slots: [Option<AbilitySlot>; 12],
    pub global_cooldown: f32,
    pub global_cooldown_duration: f32,
    pub combo_points: u32,
    pub max_combo_points: u32,
    pub combo_decay_timer: f32,
}

#[derive(Debug, Clone)]
pub struct AbilitySlot {
    pub ability_id: String,
    pub cooldown_remaining: f32,
    pub charges: u32,
    pub max_charges: u32,
    pub charge_regeneration_time: f32,
    pub last_charge_time: f32,
}

impl AbilitySystem {
    pub fn can_use_ability(&self, slot: usize) -> bool {
        if let Some(ability_slot) = &self.hotbar_slots[slot] {
            self.global_cooldown <= 0.0 && 
            ability_slot.cooldown_remaining <= 0.0 && 
            ability_slot.charges > 0
        } else { false }
    }
    
    pub fn use_ability(&mut self, slot: usize, ability_data: &AbilityData) -> bool {
        if self.can_use_ability(slot) {
            if let Some(ability_slot) = &mut self.hotbar_slots[slot] {
                ability_slot.charges -= 1;
                ability_slot.cooldown_remaining = ability_data.cooldown;
                self.global_cooldown = self.global_cooldown_duration;
                true
            } else { false }
        } else { false }
    }
    
    pub fn add_combo_points(&mut self, points: u32) {
        self.combo_points = (self.combo_points + points).min(self.max_combo_points);
        self.combo_decay_timer = COMBO_DECAY_TIME;
    }
    
    pub fn consume_combo_points(&mut self) -> u32 {
        let points = self.combo_points;
        self.combo_points = 0;
        points
    }
}
```

### StatusEffects Component

```rust
#[derive(Component, Debug)]
pub struct StatusEffects {
    pub active_effects: HashMap<String, StatusEffect>,
    pub immunity_effects: HashSet<StatusEffectType>,
    pub max_stacks: HashMap<StatusEffectType, u32>,
}

#[derive(Debug, Clone)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub remaining_time: f32,
    pub stacks: u32,
    pub source: Entity,
    pub tick_interval: f32,
    pub time_since_last_tick: f32,
    pub effect_data: StatusEffectData,
}

#[derive(Debug, Clone)]
pub enum StatusEffectType {
    Buff,
    Debuff,
    DoT,        // Damage over Time
    HoT,        // Heal over Time
    Stun,
    Silence,
    Root,
    Fear,
    StatModifier,
}

#[derive(Debug, Clone)]
pub enum StatusEffectData {
    StatModifier { stat: StatType, modifier: f32 },
    DamageOverTime { damage_per_tick: f32, damage_type: DamageType },
    HealOverTime { heal_per_tick: f32 },
    CrowdControl { cc_type: CrowdControlType },
}

impl StatusEffects {
    pub fn add_effect(&mut self, effect: StatusEffect) {
        let effect_id = format!("{}_{}", effect.effect_type, effect.source.index());
        
        // Check for existing effect and handle stacking
        if let Some(existing) = self.active_effects.get_mut(&effect_id) {
            self.handle_stacking(existing, effect);
        } else {
            self.active_effects.insert(effect_id, effect);
        }
    }
    
    pub fn remove_effect(&mut self, effect_id: &str) {
        self.active_effects.remove(effect_id);
    }
    
    pub fn has_immunity(&self, effect_type: StatusEffectType) -> bool {
        self.immunity_effects.contains(&effect_type)
    }
    
    pub fn get_stat_modifier(&self, stat: StatType) -> f32 {
        self.active_effects.values()
            .filter_map(|effect| {
                if let StatusEffectData::StatModifier { stat: effect_stat, modifier } = &effect.effect_data {
                    if *effect_stat == stat { Some(modifier * effect.stacks as f32) } else { None }
                } else { None }
            })
            .sum()
    }
    
    fn handle_stacking(&mut self, existing: &mut StatusEffect, new: StatusEffect) {
        match existing.effect_type {
            StatusEffectType::Buff | StatusEffectType::Debuff => {
                // Refresh duration, increase stacks
                existing.remaining_time = existing.duration;
                let max_stacks = self.max_stacks.get(&existing.effect_type).unwrap_or(&5);
                existing.stacks = (existing.stacks + new.stacks).min(*max_stacks);
            },
            StatusEffectType::DoT | StatusEffectType::HoT => {
                // Refresh duration, don't stack damage
                existing.remaining_time = existing.duration;
            },
            _ => {
                // Most other effects just refresh duration
                existing.remaining_time = existing.duration;
            }
        }
    }
}
```

## Core Combat Systems

### Targeting System

```rust
pub fn targeting_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&mut TargetingSystem, &Transform, &CombatStats), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &CombatStats), (With<Enemy>, Without<Player>)>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>, Without<Enemy>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut targeting, player_transform, _)) = player_query.single_mut() {
        let player_pos = player_transform.translation;
        
        // Tab targeting
        if keyboard.just_pressed(KeyCode::Tab) {
            handle_tab_targeting(&mut targeting, player_pos, &enemy_query);
        }
        
        // Click targeting
        if mouse.just_pressed(MouseButton::Left) {
            if let Ok(camera_transform) = camera_query.single() {
                if let Ok(window) = windows.single() {
                    handle_click_targeting(&mut targeting, camera_transform, window, &enemy_query);
                }
            }
        }
        
        // Validate current target
        validate_current_target(&mut targeting, player_pos, &enemy_query);
    }
}

fn handle_tab_targeting(
    targeting: &mut TargetingSystem,
    player_pos: Vec3,
    enemy_query: &Query<(Entity, &Transform, &CombatStats), (With<Enemy>, Without<Player>)>,
) {
    // Find all valid targets within range
    let mut valid_targets: Vec<(Entity, f32)> = enemy_query
        .iter()
        .filter_map(|(entity, transform, combat_stats)| {
            let distance = player_pos.distance(transform.translation);
            if distance <= targeting.max_targeting_range && combat_stats.current_health > 0.0 {
                Some((entity, distance))
            } else { None }
        })
        .collect();
    
    // Sort by distance
    valid_targets.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    if !valid_targets.is_empty() {
        // Find next target in cycle
        let current_index = targeting.current_target
            .and_then(|current| valid_targets.iter().position(|(entity, _)| *entity == current))
            .unwrap_or(valid_targets.len() - 1);
        
        let next_index = (current_index + 1) % valid_targets.len();
        targeting.set_target(Some(valid_targets[next_index].0), TargetType::Enemy);
    }
}

fn handle_click_targeting(
    targeting: &mut TargetingSystem,
    camera_transform: &Transform,
    window: &Window,
    enemy_query: &Query<(Entity, &Transform, &CombatStats), (With<Enemy>, Without<Player>)>,
) {
    if let Some(cursor_pos) = window.cursor_position() {
        // Convert screen coordinates to world ray
        // This is simplified - real implementation would use camera projection
        let world_ray = screen_to_world_ray(cursor_pos, camera_transform, window);
        
        // Find closest intersecting enemy
        let mut closest_hit: Option<(Entity, f32)> = None;
        
        for (entity, transform, combat_stats) in enemy_query.iter() {
            if combat_stats.current_health > 0.0 {
                if let Some(distance) = ray_sphere_intersection(world_ray, transform.translation, 1.0) {
                    if closest_hit.is_none() || distance < closest_hit.unwrap().1 {
                        closest_hit = Some((entity, distance));
                    }
                }
            }
        }
        
        if let Some((entity, _)) = closest_hit {
            targeting.set_target(Some(entity), TargetType::Enemy);
        }
    }
}
```

### Auto-Attack System

```rust
pub fn auto_attack_system(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut AutoAttackSystem, &TargetingSystem, &Transform, &CombatStats), With<Player>>,
    mut target_query: Query<(Entity, &Transform, &mut CombatStats), (With<Enemy>, Without<Player>)>,
    mut animation_events: EventWriter<AnimationEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let dt = time.delta_seconds();
    
    for (player_entity, mut auto_attack, targeting, player_transform, player_stats) in player_query.iter_mut() {
        auto_attack.time_since_last_attack += dt;
        
        // Check if we can and should auto-attack
        if auto_attack.can_attack() {
            if let Some(target_entity) = targeting.current_target {
                if let Ok((_, target_transform, target_stats)) = target_query.get(target_entity) {
                    let distance = player_transform.translation.distance(target_transform.translation);
                    
                    // Check range and line of sight
                    if distance <= auto_attack.weapon_range && targeting.has_line_of_sight(
                        player_transform.translation, 
                        target_transform.translation
                    ) {
                        // Initiate attack
                        initiate_auto_attack(
                            &mut auto_attack,
                            player_entity,
                            target_entity,
                            player_stats,
                            &mut animation_events,
                            &mut damage_events,
                        );
                    }
                }
            }
        }
    }
}

fn initiate_auto_attack(
    auto_attack: &mut AutoAttackSystem,
    attacker: Entity,
    target: Entity,
    attacker_stats: &CombatStats,
    animation_events: &mut EventWriter<AnimationEvent>,
    damage_events: &mut EventWriter<DamageEvent>,
) {
    auto_attack.reset_attack_timer();
    
    // Calculate damage
    let (damage, is_critical) = auto_attack.calculate_damage(attacker_stats);
    
    // Trigger attack animation
    animation_events.send(AnimationEvent {
        entity: attacker,
        animation: auto_attack.animation_name.clone(),
        looping: false,
        speed: 1.0,
    });
    
    // Schedule damage application (delayed by hit timing)
    damage_events.send(DamageEvent {
        attacker,
        target,
        damage,
        damage_type: auto_attack.damage_type,
        is_critical,
        delay: auto_attack.hit_timing,
    });
}
```

### Ability System

```rust
pub fn ability_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(Entity, &mut AbilitySystem, &TargetingSystem, &mut CombatStats), With<Player>>,
    ability_data: Res<AbilityDatabase>,
    mut ability_events: EventWriter<AbilityUsedEvent>,
) {
    let dt = time.delta_seconds();
    
    for (player_entity, mut abilities, targeting, mut combat_stats) in player_query.iter_mut() {
        // Update cooldowns and global cooldown
        abilities.global_cooldown = (abilities.global_cooldown - dt).max(0.0);
        
        for slot in &mut abilities.hotbar_slots {
            if let Some(ability_slot) = slot {
                ability_slot.cooldown_remaining = (ability_slot.cooldown_remaining - dt).max(0.0);
                
                // Handle charge regeneration
                if ability_slot.charges < ability_slot.max_charges {
                    ability_slot.last_charge_time += dt;
                    if ability_slot.last_charge_time >= ability_slot.charge_regeneration_time {
                        ability_slot.charges += 1;
                        ability_slot.last_charge_time = 0.0;
                    }
                }
            }
        }
        
        // Update combo point decay
        if abilities.combo_points > 0 {
            abilities.combo_decay_timer -= dt;
            if abilities.combo_decay_timer <= 0.0 {
                abilities.combo_points = 0;
            }
        }
        
        // Handle ability input (1-0 keys for hotbar slots 0-9)
        for (key_index, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5,
                                 KeyCode::Digit6, KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9, KeyCode::Digit0].iter().enumerate() {
            if keyboard.just_pressed(*key) && abilities.can_use_ability(key_index) {
                if let Some(ability_slot) = &abilities.hotbar_slots[key_index] {
                    if let Some(ability_data) = ability_data.get(&ability_slot.ability_id) {
                        // Check resource costs
                        if can_afford_ability(&combat_stats, ability_data) {
                            // Use ability
                            abilities.use_ability(key_index, ability_data);
                            consume_ability_resources(&mut combat_stats, ability_data);
                            
                            // Send ability used event
                            ability_events.send(AbilityUsedEvent {
                                caster: player_entity,
                                ability_id: ability_slot.ability_id.clone(),
                                target: targeting.current_target,
                                combo_points: abilities.combo_points,
                            });
                        }
                    }
                }
            }
        }
    }
}

fn can_afford_ability(combat_stats: &CombatStats, ability_data: &AbilityData) -> bool {
    combat_stats.current_mana >= ability_data.mana_cost &&
    combat_stats.current_stamina >= ability_data.stamina_cost &&
    combat_stats.current_focus >= ability_data.focus_cost
}

fn consume_ability_resources(combat_stats: &mut CombatStats, ability_data: &AbilityData) {
    combat_stats.current_mana -= ability_data.mana_cost;
    combat_stats.current_stamina -= ability_data.stamina_cost;
    combat_stats.current_focus -= ability_data.focus_cost;
}
```

## Event System for Combat

```rust
#[derive(Event)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: f32,
    pub damage_type: DamageType,
    pub is_critical: bool,
    pub delay: f32,
}

#[derive(Event)]
pub struct AbilityUsedEvent {
    pub caster: Entity,
    pub ability_id: String,
    pub target: Option<Entity>,
    pub combo_points: u32,
}

#[derive(Event)]
pub struct StatusEffectAppliedEvent {
    pub source: Entity,
    pub target: Entity,
    pub effect: StatusEffect,
}

#[derive(Event)]
pub struct AnimationEvent {
    pub entity: Entity,
    pub animation: String,
    pub looping: bool,
    pub speed: f32,
}

// Event handling systems
pub fn damage_event_system(
    time: Res<Time>,
    mut damage_events: EventReader<DamageEvent>,
    mut delayed_damage: Local<Vec<(DamageEvent, f32)>>,
    mut target_query: Query<&mut CombatStats>,
    mut visual_events: EventWriter<DamageVisualEvent>,
) {
    let dt = time.delta_seconds();
    
    // Add new damage events to delayed list
    for event in damage_events.read() {
        delayed_damage.push((event.clone(), event.delay));
    }
    
    // Process delayed damage
    let mut i = 0;
    while i < delayed_damage.len() {
        delayed_damage[i].1 -= dt;
        
        if delayed_damage[i].1 <= 0.0 {
            let damage_event = delayed_damage.remove(i);
            
            // Apply damage
            if let Ok(mut target_stats) = target_query.get_mut(damage_event.0.target) {
                let actual_damage = target_stats.take_damage(damage_event.0.damage, damage_event.0.damage_type);
                
                // Send visual feedback event
                visual_events.send(DamageVisualEvent {
                    target: damage_event.0.target,
                    damage: actual_damage,
                    is_critical: damage_event.0.is_critical,
                    damage_type: damage_event.0.damage_type,
                });
            }
        } else {
            i += 1;
        }
    }
}
```

This architecture provides a solid foundation for implementing engaging combat mechanics while maintaining the modularity and performance characteristics required for an MMORPG. The component-based design allows for easy extension and modification as the system evolves.