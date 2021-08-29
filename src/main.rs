use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline};
use bevy::render::shader::{ShaderStage, ShaderStages};

mod camera;

fn create_triangle() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![2, 1, 0])));
    mesh
}

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;
// layout(location = 1) in vec3 Vertex_Color;
// layout(location = 0) out vec3 v_color;
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
    // v_color = Vertex_Color;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
// layout(location = 0) in vec3 v_color;
void main() {
    o_Target = vec4(0.5, 0.3, 0.3, 1.0);
}
"#;

fn setup(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0),
        ..Default::default()
    });

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    let mesh_handle = meshes.add(create_triangle());
    // let mesh_handle = meshes.add(Cube::new(1.).into());
    let _material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    let mesh_bundle = MeshBundle {
        mesh: mesh_handle,
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle,
        )]),
        // material: material_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };
    commands
        .spawn_bundle((
            Transform {
                translation: Vec3::new(-0.5, 0., 0.),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_bundle(mesh_bundle);
        })
        .insert(Rotates);
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands
        .spawn_bundle((Transform::from_xyz(0., 0., 0.), GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("monkey.gltf#Scene0"));
        })
        .insert(Rotates);
}

struct Rotates;

fn rotator(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in query.iter_mut() {
        *transform = Transform::from_rotation(Quat::from_rotation_y(
            (4. * std::f32::consts::PI / 20.) * time.delta_seconds(),
        )) * *transform;
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy game".into(),
            width: 500.,
            height: 300.,
            vsync: true,
            decorations: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.2)))
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(rotator)
        .add_plugin(camera::FPSCameraPlugin { sensitivity: 0.5 })
        .run();
}
