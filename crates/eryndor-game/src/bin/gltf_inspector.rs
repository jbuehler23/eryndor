/*!
 * GLTF Inspector - Comprehensive GLTF/GLB file analysis tool
 * 
 * This utility inspects and displays detailed information about GLTF/GLB files including:
 * - All animations with names, indices, and durations
 * - All scenes and their hierarchies
 * - All meshes and materials
 * - All nodes and their relationships
 * - Asset metadata and structure
 * 
 * Usage: cargo run --bin gltf_inspector [path_to_gltf_file]
 * Example: cargo run --bin gltf_inspector assets/models/character.glb
 */

use bevy::prelude::*;
use bevy::gltf::Gltf;
use bevy::asset::LoadState;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let gltf_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // Default to Knight.glb if no argument provided
        "KayKit_Adventurers_1.0_FREE/Characters/gltf/Knight.glb".to_string()
    };

    println!("🔍 GLTF Inspector - Analyzing: {}", gltf_path);
    println!("{}", "=".repeat(80));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false, // Hide window for CLI tool
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GltfPath(gltf_path))
        .add_systems(Startup, setup)
        .add_systems(Update, inspect_gltf_comprehensive)
        .run();
}

#[derive(Resource)]
struct GltfPath(String);

#[derive(Resource)]
struct GltfHandle(Handle<Gltf>);

#[derive(Resource)]
struct LoadingTimer {
    start_time: Instant,
    timeout: Duration,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, gltf_path: Res<GltfPath>) {
    commands.spawn(Camera3d::default());
    
    // Load the GLTF file
    let gltf_handle: Handle<Gltf> = asset_server.load(&gltf_path.0);
    commands.insert_resource(GltfHandle(gltf_handle));
    
    // Start loading timer (10 second timeout)
    commands.insert_resource(LoadingTimer {
        start_time: Instant::now(),
        timeout: Duration::from_secs(10),
    });
}

fn inspect_gltf_comprehensive(
    gltf_handle: Res<GltfHandle>,
    gltf_assets: Res<Assets<Gltf>>,
    animation_clips: Res<Assets<AnimationClip>>,
    scenes: Res<Assets<Scene>>,
    meshes: Res<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    loading_timer: Res<LoadingTimer>,
) {
    // Check for timeout
    if loading_timer.start_time.elapsed() > loading_timer.timeout {
        let load_state = asset_server.load_state(&gltf_handle.0);
        match load_state {
            LoadState::Failed(_) => {
                println!("❌ Failed to load GLTF file!");
                println!("   Check that the file path exists and is a valid GLTF/GLB file.");
            },
            LoadState::NotLoaded => {
                println!("❌ GLTF file not found!");
                println!("   Check that the file path is correct.");
            },
            _ => {
                println!("⏰ Loading timeout - file may be too large or have issues.");
                println!("   Load state: {:?}", load_state);
            }
        }
        std::process::exit(1);
    }
    
    if let Some(gltf) = gltf_assets.get(&gltf_handle.0) {
        println!("✅ GLTF file loaded successfully!");
        println!();

        // 1. ANIMATIONS ANALYSIS
        inspect_animations(gltf, &animation_clips);
        
        // 2. SCENES ANALYSIS
        inspect_scenes(gltf, &scenes);
        
        // 3. MESHES ANALYSIS
        inspect_meshes(gltf, &meshes);
        
        // 4. MATERIALS ANALYSIS
        inspect_materials(gltf, &materials);
        
        // 5. NODES ANALYSIS
        inspect_nodes(gltf);
        
        // 6. SUMMARY
        print_summary(gltf);
        
        // Exit after inspection
        println!("\n🎯 Inspection complete!");
        println!("{}", "=".repeat(80));
        std::process::exit(0);
    }
}

fn inspect_animations(gltf: &Gltf, animation_clips: &Assets<AnimationClip>) {
    println!("🎬 ANIMATIONS ({} total)", gltf.named_animations.len());
    println!("{}", "-".repeat(60));
    
    if gltf.named_animations.is_empty() {
        println!("  No animations found.");
        println!();
        return;
    }

    // Create sorted list of animations for consistent indexing
    let mut animations: Vec<_> = gltf.named_animations.iter().collect();
    animations.sort_by(|a, b| a.0.cmp(&b.0));

    for (index, (name, handle)) in animations.iter().enumerate() {
        if let Some(clip) = animation_clips.get(*handle) {
            println!("  Animation{:2}: '{}' - Duration: {:.2}s", 
                index, name, clip.duration());
                
            // Highlight common game animation types
            match name.to_lowercase().as_str() {
                n if n.contains("idle") => println!("    🔸 IDLE animation"),
                n if n.contains("walk") => println!("    🚶 WALK animation"),
                n if n.contains("run") => println!("    🏃 RUN animation"),
                n if n.contains("jump") => println!("    🦘 JUMP animation"),
                n if n.contains("attack") => println!("    ⚔️  ATTACK animation"),
                n if n.contains("death") => println!("    💀 DEATH animation"),
                n if n.contains("hit") => println!("    💥 HIT animation"),
                _ => {}
            }
        } else {
            println!("  Animation{:2}: '{}' - Loading...", index, name);
        }
    }
    println!();
}

fn inspect_scenes(gltf: &Gltf, scenes: &Assets<Scene>) {
    println!("🎭 SCENES ({} total)", gltf.named_scenes.len());
    println!("{}", "-".repeat(60));
    
    if gltf.named_scenes.is_empty() {
        println!("  No named scenes found.");
        println!();
        return;
    }

    for (index, (name, handle)) in gltf.named_scenes.iter().enumerate() {
        if let Some(_scene) = scenes.get(handle) {
            println!("  Scene{}: '{}'", index, name);
            // TODO: Could add scene hierarchy inspection here
        } else {
            println!("  Scene{}: '{}' - Loading...", index, name);
        }
    }
    println!();
}

fn inspect_meshes(gltf: &Gltf, _meshes: &Assets<Mesh>) {
    println!("🔺 MESHES ({} total)", gltf.named_meshes.len());
    println!("{}", "-".repeat(60));
    
    if gltf.named_meshes.is_empty() {
        println!("  No named meshes found.");
        println!();
        return;
    }

    for (name, _handle) in &gltf.named_meshes {
        println!("  Mesh: '{}'", name);
        // Note: GLTF meshes are different from Bevy meshes, detailed inspection would require more complex handling
    }
    println!();
}

fn inspect_materials(gltf: &Gltf, materials: &Assets<StandardMaterial>) {
    println!("🎨 MATERIALS ({} total)", gltf.named_materials.len());
    println!("{}", "-".repeat(60));
    
    if gltf.named_materials.is_empty() {
        println!("  No named materials found.");
        println!();
        return;
    }

    for (name, handle) in &gltf.named_materials {
        if let Some(material) = materials.get(handle) {
            println!("  Material: '{}'", name);
            println!("    Base Color: {:?}", material.base_color);
            println!("    Metallic: {:.2}, Roughness: {:.2}", 
                material.metallic, material.perceptual_roughness);
            
            if material.base_color_texture.is_some() {
                println!("    Has base color texture");
            }
            if material.normal_map_texture.is_some() {
                println!("    Has normal map");
            }
        } else {
            println!("  Material: '{}' - Loading...", name);
        }
    }
    println!();
}

fn inspect_nodes(gltf: &Gltf) {
    println!("🌳 NODES ({} total)", gltf.named_nodes.len());
    println!("{}", "-".repeat(60));
    
    if gltf.named_nodes.is_empty() {
        println!("  No named nodes found.");
        println!();
        return;
    }

    for (name, _handle) in &gltf.named_nodes {
        println!("  Node: '{}'", name);
        
        // Identify node types by name patterns
        match name.to_lowercase().as_str() {
            n if n.contains("armature") || n.contains("skeleton") => 
                println!("    🦴 Skeletal/Armature node"),
            n if n.contains("bone") || n.contains("joint") => 
                println!("    🔗 Bone/Joint node"),
            n if n.contains("mesh") => 
                println!("    🔺 Mesh node"),
            n if n.contains("camera") => 
                println!("    📷 Camera node"),
            n if n.contains("light") => 
                println!("    💡 Light node"),
            _ => {}
        }
    }
    println!();
}

fn print_summary(gltf: &Gltf) {
    println!("📊 SUMMARY");
    println!("{}", "-".repeat(60));
    println!("  Animations: {}", gltf.named_animations.len());
    println!("  Scenes: {}", gltf.named_scenes.len());
    println!("  Meshes: {}", gltf.named_meshes.len());
    println!("  Materials: {}", gltf.named_materials.len());
    println!("  Nodes: {}", gltf.named_nodes.len());
    
    // Provide usage tips
    if !gltf.named_animations.is_empty() {
        println!("\n💡 Animation Usage Tips:");
        println!("  Use GltfAssetLabel::Animation(INDEX) to load specific animations");
        println!("  Example: asset_server.load(GltfAssetLabel::Animation(0).from_asset(\"your_file.glb\"))");
    }
    
    if !gltf.named_scenes.is_empty() {
        println!("\n💡 Scene Usage Tips:");
        println!("  Use GltfAssetLabel::Scene(INDEX) to load specific scenes");
        println!("  Example: asset_server.load(GltfAssetLabel::Scene(0).from_asset(\"your_file.glb\"))");
    }
}