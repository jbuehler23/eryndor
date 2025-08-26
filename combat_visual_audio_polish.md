# Combat Visual and Audio Polish Systems

## Overview

Visual and audio feedback transforms mechanical combat calculations into visceral, satisfying gameplay experiences. This document outlines the systems that provide immediate, clear, and impactful feedback for all combat actions.

## Visual Feedback Systems

### Damage Display System

```rust
#[derive(Component, Debug)]
pub struct DamageNumberSystem {
    pub active_numbers: Vec<DamageNumber>,
    pub number_pool: Vec<DamageNumber>, // Object pooling for performance
    pub style_config: DamageNumberStyles,
    pub physics_config: DamageNumberPhysics,
}

#[derive(Debug, Clone)]
pub struct DamageNumber {
    pub value: f32,
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub scale: f32,
    pub color: Color,
    pub font_size: f32,
    pub is_critical: bool,
    pub damage_type: DamageType,
    pub animation_phase: DamageAnimationPhase,
}

#[derive(Debug, Clone)]
pub enum DamageAnimationPhase {
    Spawn,      // Initial appearance with scale-up
    Rise,       // Floating upward movement
    Fade,       // Transparency fade-out
    Despawn,    // Cleanup phase
}

#[derive(Debug)]
pub struct DamageNumberStyles {
    pub normal_color: Color,
    pub critical_color: Color,
    pub healing_color: Color,
    pub magic_colors: HashMap<DamageType, Color>,
    pub font_sizes: DamageFontSizes,
    pub animation_curves: DamageAnimationCurves,
}

#[derive(Debug)]
pub struct DamageFontSizes {
    pub base_size: f32,
    pub critical_multiplier: f32,
    pub healing_multiplier: f32,
    pub overkill_multiplier: f32,
}
```

### Damage Number Implementation

**Visual Design Principles**
- **Immediate Recognition**: Color coding allows instant damage type identification
- **Impact Communication**: Size and animation intensity reflect damage magnitude
- **Combat Clarity**: Numbers don't obscure important visual information
- **Performance Optimization**: Object pooling prevents framerate drops

**Damage Number Behaviors**
- **Normal Damage**: White text, standard size, gentle upward float
- **Critical Hits**: Gold text, 1.5x size, explosive spawn with screen shake
- **Healing**: Green text, downward float, soft glow effect
- **Magic Damage**: Color-coded by element, particle trail effects
- **Overkill**: Red text, 2x size, dramatic screen shake

```rust
pub fn damage_number_system(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_events: EventReader<DamageVisualEvent>,
    mut damage_numbers: Query<&mut DamageNumberSystem>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_seconds();
    
    // Process new damage events
    for event in damage_events.read() {
        spawn_damage_number(event, &mut commands, &asset_server);
    }
    
    // Update existing damage numbers
    for mut damage_system in damage_numbers.iter_mut() {
        update_damage_numbers(&mut damage_system, dt);
    }
}

fn spawn_damage_number(
    event: &DamageVisualEvent,
    commands: &mut Commands,
    asset_server: &AssetServer,
) {
    let number_style = get_damage_number_style(event.damage, event.is_critical, event.damage_type);
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{:.0}", event.damage),
                TextStyle {
                    font: asset_server.load("fonts/combat_numbers.ttf"),
                    font_size: number_style.font_size,
                    color: number_style.color,
                },
            ),
            transform: Transform::from_translation(event.world_position + Vec3::Y * 2.0),
            ..default()
        },
        DamageNumber {
            value: event.damage,
            position: event.world_position,
            velocity: Vec3::new(0.0, 2.0, 0.0),
            lifetime: 0.0,
            max_lifetime: 2.5,
            scale: if event.is_critical { 1.5 } else { 1.0 },
            color: number_style.color,
            font_size: number_style.font_size,
            is_critical: event.is_critical,
            damage_type: event.damage_type,
            animation_phase: DamageAnimationPhase::Spawn,
        },
        CombatUIElement, // Marker component
    ));
}
```

### Screen Effects System

```rust
#[derive(Resource, Debug)]
pub struct ScreenEffectSystem {
    pub shake_intensity: f32,
    pub shake_duration: f32,
    pub shake_frequency: f32,
    pub flash_color: Color,
    pub flash_intensity: f32,
    pub flash_duration: f32,
    pub zoom_pulse_intensity: f32,
    pub zoom_pulse_duration: f32,
    pub effect_queue: Vec<ScreenEffect>,
}

#[derive(Debug, Clone)]
pub struct ScreenEffect {
    pub effect_type: ScreenEffectType,
    pub intensity: f32,
    pub duration: f32,
    pub color: Option<Color>,
    pub easing: EasingFunction,
}

#[derive(Debug, Clone)]
pub enum ScreenEffectType {
    Shake,
    Flash,
    ZoomPulse,
    ColorOverlay,
    Distortion,
    ChromaticAberration,
}

#[derive(Debug, Clone)]
pub enum EasingFunction {
    Linear,
    EaseOut,
    EaseIn,
    Bounce,
    Elastic,
}
```

**Screen Effect Triggers**
- **Critical Hits**: Medium screen shake + brief yellow flash
- **Player Near Death**: Red screen overlay with pulse effect
- **Spell Cast**: Elemental color flash matching spell type
- **Large Damage**: Screen shake intensity scales with damage percentage
- **Perfect Block**: Brief white flash with zoom pulse

### Particle Effects Framework

```rust
#[derive(Component, Debug)]
pub struct ParticleEffectSystem {
    pub active_effects: Vec<ParticleEffect>,
    pub effect_templates: HashMap<String, ParticleEffectTemplate>,
    pub max_particles_per_effect: u32,
    pub global_particle_limit: u32,
    pub performance_scaling: f32,
}

#[derive(Debug, Clone)]
pub struct ParticleEffect {
    pub template_id: String,
    pub position: Vec3,
    pub particles: Vec<Particle>,
    pub emission_rate: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub scale_modifier: f32,
    pub color_modifier: Color,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
    pub color: Color,
    pub rotation: f32,
    pub angular_velocity: f32,
}

#[derive(Debug, Clone)]
pub struct ParticleEffectTemplate {
    pub name: String,
    pub emission_shape: EmissionShape,
    pub particle_count: (u32, u32), // Min, Max
    pub lifetime_range: (f32, f32),
    pub size_range: (f32, f32),
    pub velocity_range: (Vec3, Vec3),
    pub color_gradient: Vec<(f32, Color)>, // Time, Color pairs
    pub physics_properties: ParticlePhysics,
    pub rendering_properties: ParticleRendering,
}
```

**Combat Particle Effects**
- **Sword Strikes**: Metallic spark particles along blade trajectory
- **Fire Magic**: Burning ember particles with heat distortion
- **Ice Magic**: Crystalline shard particles with frost trails
- **Healing**: Soft golden motes floating upward
- **Critical Hits**: Explosive burst with radial particle spray
- **Death**: Soul essence particles dissipating upward

### Animation Integration

```rust
#[derive(Component, Debug)]
pub struct CombatAnimationController {
    pub current_state: CombatAnimationState,
    pub state_machine: CombatStateMachine,
    pub blend_weights: HashMap<String, f32>,
    pub animation_events: Vec<AnimationEvent>,
    pub weapon_trail_config: WeaponTrailConfig,
}

#[derive(Debug, Clone)]
pub enum CombatAnimationState {
    Idle,
    WindUp,      // Pre-attack preparation
    Strike,      // Actual attack execution
    Recovery,    // Post-attack recovery
    Block,       // Defensive stance
    Dodge,       // Evasive maneuver
    Stagger,     // Hit reaction
    Death,       // Death animation
}

#[derive(Debug)]
pub struct WeaponTrailConfig {
    pub trail_enabled: bool,
    pub trail_color: Color,
    pub trail_width: f32,
    pub trail_length: f32,
    pub trail_fade_time: f32,
    pub blade_glow_intensity: f32,
}
```

**Animation-Driven Effects**
- **Weapon Trails**: Dynamic weapon glow and particle trails during attacks
- **Impact Frames**: Brief pause on hit for impact sensation
- **Anticipation**: Wind-up animations telegraph attacks while building tension
- **Follow-Through**: Recovery animations prevent animation canceling exploits
- **Reactive Animations**: Character reactions to different damage types

## Audio System Architecture

### 3D Positional Audio Framework

```rust
#[derive(Resource, Debug)]
pub struct CombatAudioSystem {
    pub audio_sources: HashMap<Entity, Vec<AudioSource>>,
    pub sound_pools: HashMap<String, SoundPool>,
    pub audio_config: CombatAudioConfig,
    pub mixer_channels: HashMap<AudioChannel, f32>,
    pub occlusion_system: AudioOcclusionSystem,
}

#[derive(Debug)]
pub struct CombatAudioConfig {
    pub max_simultaneous_sounds: u32,
    pub distance_attenuation: f32,
    pub doppler_effect: bool,
    pub environmental_reverb: bool,
    pub dynamic_range_compression: f32,
    pub audio_quality_scaling: AudioQualityLevel,
}

#[derive(Debug)]
pub enum AudioChannel {
    Master,
    Combat,
    Environment,
    UI,
    Music,
    Voice,
}

#[derive(Debug)]
pub struct SoundPool {
    pub sound_variants: Vec<Handle<AudioSource>>,
    pub last_played: f32,
    pub min_replay_delay: f32,
    pub randomization: SoundRandomization,
}

#[derive(Debug)]
pub struct SoundRandomization {
    pub pitch_variance: (f32, f32),
    pub volume_variance: (f32, f32),
    pub variant_selection: VariantSelectionMode,
}

#[derive(Debug)]
pub enum VariantSelectionMode {
    Random,
    Sequential,
    Weighted(Vec<f32>),
    AntiRepeat, // Prevents same variant playing consecutively
}
```

### Combat Audio Categories

**Weapon Sound Design**
```rust
pub struct WeaponAudioProfile {
    pub swing_sounds: SoundPool,      // Wind-cutting sounds
    pub impact_sounds: HashMap<MaterialType, SoundPool>,
    pub critical_sounds: SoundPool,   // Special critical hit sounds
    pub block_sounds: SoundPool,      // Defensive parry/block sounds
    pub weapon_material: WeaponMaterial,
}

pub enum MaterialType {
    Flesh,
    Metal,
    Leather,
    Stone,
    Wood,
    Cloth,
    Magical,
}

pub enum WeaponMaterial {
    Steel,     // Sharp, metallic impacts
    Iron,      // Heavier, duller impacts
    Silver,    // Bright, ringing impacts
    Wood,      // Thudding, organic impacts
    Crystal,   // Chiming, magical impacts
    Bone,      // Hollow, eerie impacts
}
```

**Magic Sound Design**
- **Fire Magic**: Crackling flames, explosive bursts, sizzling impacts
- **Ice Magic**: Crystalline chimes, freezing sounds, brittle shatters
- **Lightning**: Electric arcs, thunderclaps, energy crackles
- **Shadow Magic**: Whispers, void echoes, dark resonance
- **Nature Magic**: Wind rushing, earth rumbling, water flowing
- **Arcane Magic**: Reality warping, dimensional tears, pure energy

**Environmental Audio**
- **Footsteps**: Different sounds for various terrain types
- **Ambient Combat**: Crowd cheers, distant battles, environmental hazards
- **Weather Integration**: Rain affecting fire magic sounds, wind affecting projectiles
- **Architectural Acoustics**: Cathedral reverb, cave echoes, outdoor openness

### Dynamic Music System

```rust
#[derive(Resource, Debug)]
pub struct DynamicMusicSystem {
    pub current_track: Option<Handle<AudioSource>>,
    pub combat_music: CombatMusicConfig,
    pub exploration_music: ExplorationMusicConfig,
    pub transition_system: MusicTransitionSystem,
    pub intensity_tracking: CombatIntensityTracker,
}

#[derive(Debug)]
pub struct CombatMusicConfig {
    pub low_intensity: Vec<Handle<AudioSource>>,
    pub medium_intensity: Vec<Handle<AudioSource>>,
    pub high_intensity: Vec<Handle<AudioSource>>,
    pub boss_themes: HashMap<String, Handle<AudioSource>>,
    pub victory_stingers: Vec<Handle<AudioSource>>,
    pub defeat_themes: Vec<Handle<AudioSource>>,
}

#[derive(Debug)]
pub struct CombatIntensityTracker {
    pub current_intensity: f32,
    pub factors: IntensityFactors,
    pub smoothing_rate: f32,
    pub threshold_low: f32,
    pub threshold_medium: f32,
    pub threshold_high: f32,
}

#[derive(Debug)]
pub struct IntensityFactors {
    pub player_health_percentage: f32,
    pub enemies_in_combat: u32,
    pub damage_per_second: f32,
    pub ability_usage_rate: f32,
    pub critical_hits_per_minute: f32,
    pub combo_streak_length: u32,
}
```

**Music Transition System**
- **Exploration to Combat**: Gradual intensity buildup over 2-3 seconds
- **Combat Intensity Scaling**: Music adapts to fight difficulty and player performance
- **Victory/Defeat**: Clear musical resolution with appropriate emotional tone
- **Seamless Looping**: Combat music loops without noticeable breaks
- **Boss Encounters**: Unique themes that override standard combat music

### Audio Feedback for Combat Actions

**Immediate Feedback Audio**
```rust
pub fn combat_audio_feedback_system(
    mut audio_events: EventReader<CombatAudioEvent>,
    mut commands: Commands,
    audio_system: Res<CombatAudioSystem>,
    player_query: Query<&Transform, With<Player>>,
    target_query: Query<&Transform, (With<CombatTarget>, Without<Player>)>,
) {
    for event in audio_events.read() {
        match &event.event_type {
            CombatAudioEventType::WeaponSwing { weapon_type, position } => {
                play_3d_sound(&mut commands, &audio_system, 
                    get_weapon_swing_sound(*weapon_type), *position);
            },
            CombatAudioEventType::WeaponHit { weapon_material, target_material, is_critical, position } => {
                let sound_pool = get_impact_sound(*weapon_material, *target_material);
                let sound = if *is_critical { 
                    get_critical_impact_sound(*weapon_material, *target_material)
                } else { 
                    sound_pool.get_random_variant() 
                };
                play_3d_sound(&mut commands, &audio_system, sound, *position);
            },
            CombatAudioEventType::SpellCast { spell_school, position } => {
                play_spell_cast_sound(&mut commands, &audio_system, *spell_school, *position);
            },
            CombatAudioEventType::AbilityReady { ability_id } => {
                play_ui_sound(&mut commands, &audio_system, "ability_ready");
            },
            // ... other audio event types
        }
    }
}
```

**Audio Cue System**
- **Cooldown Complete**: Subtle chime when abilities become available
- **Resource Low**: Warning sound when mana/stamina drops below 25%
- **Incoming Attack**: Audio telegraph for enemy abilities (directional)
- **Perfect Timing**: Satisfying "ding" sound for perfect blocks/dodges
- **Combo Success**: Musical chord progression for successful combo chains

### Performance Optimization

**Audio Performance Strategies**
```rust
#[derive(Resource, Debug)]
pub struct AudioPerformanceManager {
    pub max_concurrent_sources: u32,
    pub distance_culling: f32,
    pub priority_system: AudioPrioritySystem,
    pub compression_settings: AudioCompressionConfig,
    pub streaming_config: AudioStreamingConfig,
}

#[derive(Debug)]
pub enum AudioPriority {
    Critical,   // Player actions, UI feedback
    High,       // Combat impacts, spell effects
    Medium,     // Environmental sounds, footsteps
    Low,        // Ambient audio, distant effects
    Background, // Music, atmospheric audio
}
```

**Optimization Techniques**
- **Distance-Based Culling**: Sounds beyond audible range aren't processed
- **Priority-Based Management**: Important sounds interrupt less important ones
- **Sound Pooling**: Reuse audio components to prevent memory allocation
- **Dynamic Quality Scaling**: Reduce audio quality during performance drops
- **Occlusion Culling**: Muffled audio through walls and obstacles

### Audio-Visual Synchronization

**Timing Synchronization**
- **Hit Registration**: Audio plays exactly when damage numbers appear
- **Animation Sync**: Weapon impact sounds align with contact frames
- **Spell Effects**: Audio and particle effects launch simultaneously
- **Screen Shake Integration**: Audio intensity matches visual shake intensity
- **UI Feedback**: Button press audio confirms input registration

**Multi-Sensory Combat Experience**
- **Haptic Feedback**: Controller vibration synced with audio impacts (PC gamepad support)
- **Visual-Audio Harmony**: Color temperature of visual effects matches audio tone
- **Rhythmic Combat**: Abilities and auto-attacks create natural combat rhythm
- **Environmental Storytelling**: Audio design reinforces the fantasy setting

This comprehensive audio-visual system transforms mechanical combat calculations into visceral, satisfying player experiences that provide clear feedback while maintaining immersion in the fantasy world.