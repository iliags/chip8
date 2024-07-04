use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::WindowResized,
};

use crate::DeviceContext;

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_display);
        app.add_systems(Update, update_display);
        app.add_systems(Update, fit_canvas);
    }
}

/// Resolution width.
const RES_WIDTH: u32 = 256;

/// Resolution height.
const RES_HEIGHT: u32 = 256;

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct DisplayCamera;

fn initialize_display(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    //commands.insert_resource(Display::default());

    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // render before the "main pass" camera
                //order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        DisplayCamera,
    ));

    commands.spawn((
        SpriteBundle {
            texture: image_handle.clone(),
            ..default()
        },
        Canvas,
        //HIGH_RES_LAYERS,
    ));
}

fn update_display(
    display: Res<DeviceContext>,
    mut canvases: Query<&mut Canvas>,
    mut images: ResMut<Assets<Image>>,
) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };
    let image_data = [1; (RES_HEIGHT * RES_WIDTH) as usize];
    //println!("{:?}", image_data);

    for image in images.iter_mut() {
        for data_vec in image.1.data.iter_mut() {
            *data_vec = 1;
        }
        //image.1.data = image_data.to_vec();
    }

    /*
    for mut canvas in canvases.iter_mut() {
        canvas.image = Image::new_fill(
            canvas_size,
            TextureDimension::D2,
            &image_data,
            TextureFormat::Bgra8UnormSrgb,
            RenderAssetUsages::default(),
        );
    }
    */
}

/// Scales camera projection to fit the window (integer multiples only).
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<DisplayCamera>>,
) {
    for event in resize_events.read() {
        let h_scale = event.width / RES_WIDTH as f32;
        let v_scale = event.height / RES_HEIGHT as f32;
        let mut projection = projections.single_mut();
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
