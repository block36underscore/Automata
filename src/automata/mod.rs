use std::ops::{Index, IndexMut};

use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

use self::rule::RuleContext;

pub mod material;
pub mod rule;
pub mod life;

pub const SIZE: usize = 16;

#[derive(Component, Clone, Copy)]
pub struct Field<T: Sized, const W: usize, const H: usize> {
    pub contents: [Column<T, H>; W],
}

#[derive(Copy, Clone)]
pub struct Column<T, const H: usize> (pub [T; H]);

impl <T, const H: usize> Index<usize> for Column<T, H> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index % H]
    }
}

impl <T, const H: usize> IndexMut<usize> for Column<T, H> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index % H]
    }
}

impl <T, const W: usize, const H: usize> Index<usize> for Field<T, W, H> {
    type Output = Column<T, H>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.contents[index % W]
    }
}

impl <T, const W: usize, const H: usize> IndexMut<usize> for Field<T, W, H> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.contents[index % W]
    }
}

impl <T, const W: usize, const H: usize> Index<IVec2> for Field<T, W, H> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        &self.contents[index.x as usize % W][index.y as usize % H]
    }
}


impl <T, const W: usize, const H: usize> IndexMut<IVec2> for Field<T, W, H> {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        &mut self.contents[index.x as usize % W][index.y as usize % H]
    }
}

pub type FieldAccess<T> = dyn Index<IVec2, Output = T>;
pub type FieldAccessMut<T> = dyn IndexMut<IVec2, Output = T>;


pub type ColorField<const W: usize, const H: usize> = Field<[u8; 4], W, H>;

impl <const W: usize, const H: usize> Into<Vec<u8>> for &ColorField<W, H> {
    fn into(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(W * H * 4);
        for column in self.contents {
            for pixel in column.0 {
                vec.extend_from_slice(&pixel);
            }
        }
        vec
    }
}

#[derive(Component)]
pub struct Life {
    pub field: LifeField<SIZE, SIZE>,
    pub timer: Timer,
}

impl Life {
    pub fn new() -> Self {
        Self {
            field: LifeField::new(),
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }

    pub fn render(
            time: Res<Time>,
            mut life_instances: Query<(&mut Life, &mut Handle<Image>, &RuleContext<bool>)>,
            mut textures: ResMut<Assets<Image>>,
        ) {
        for (mut life, mut image, context) in &mut life_instances {
            if life.timer.tick(time.delta()).just_finished() {
                life.field.tick(context);
                let field = life.field.render();
                let rendered_image = Image::new(
                    Extent3d {
                        width:  16,
                        height: 16,
                        depth_or_array_layers: 1,
                    }, 
                    TextureDimension::D2,
                    (&*field).into(),
                    TextureFormat::Rgba8Unorm,
                    default());
                let handle = textures.add(rendered_image);
                *image = handle;
            }
        }
    }

    pub fn setup(mut commands: Commands,
                mut textures: ResMut<Assets<Image>>
                ) {
        let life = Life::new();
        let field = life.field.render();
        let image = Image::new(
            Extent3d {
                width:  SIZE as u32,
                height: SIZE as u32,
                depth_or_array_layers: 1,
            }, 
            TextureDimension::D2,
            (&*field).into(),
            TextureFormat::Rgba8Unorm,
            default());
        let handle = textures.add(image);
    
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(300.0, 300.0)),
                    color: Color::rgb(0.5, 0.7, 0.6),
                    ..Default::default()
                },
                texture: handle,
                ..Default::default()
            },
            life,
            life::gen_life_context(),
        ));
    }

}

pub type LifeField<const W: usize, const H: usize> = Field<bool, W, H>;

impl <const W: usize, const H: usize> LifeField<W, H> {
    pub fn new() -> LifeField<W, H> {
        let mut contents = [Column([false; H]); W];
        contents[8][8] = true;
        contents[9][8] = true;
        contents[10][8] = true;
        contents[10][9] = true;
        contents[9][10] = true;
        LifeField {
            contents
        }
    }

    const WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
    const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];

    pub fn render(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(W * H * 4);
        for x in 0..W {
            for y in 0..H {
                if self.contents[x][y] {
                    vec.extend_from_slice(&Self::WHITE);
                } else {
                    vec.extend_from_slice(&Self::BLACK);
                }
            }
        }

        vec
    }

    pub fn tick(&mut self, context: &RuleContext<bool>) {
        let mut target = self.clone();
        for x in 0..(W as i32) {
            for y in 0..(H as i32) {
                context.apply(self, &mut target, IVec2::new(x, y));
            }
        }

        self.contents = target.contents;
    }

    pub fn get(&self, x: i32, y: i32) -> bool {
        self.contents[x as usize % W][y as usize % H]
    }

}

const INIT: [u8; 4] = [0xFF, 0x00, 0x00, 0xFF];

pub fn gen_test<const W: usize, const H: usize>() -> Box<ColorField<W, H>> {
    let mut field = ColorField {
        contents: [Column([INIT; H]); W]
    };
    
    for x in 0..16 {
        for y in 0..16 {
            field.contents[x][y] = [(x * SIZE) as u8, (y * SIZE) as u8, 0, 0xFF];
        }
    }

    Box::new(field)
}

pub fn test(mut commands: Commands,
            mut textures: ResMut<Assets<Image>>
            ) {
    let field = gen_test::<16, 16>();
    let image = Image::new(
        Extent3d {
            width:  SIZE as u32,
            height: SIZE as u32,
            depth_or_array_layers: 1,
        }, 
        TextureDimension::D2,
        (&*field).into(),
        TextureFormat::Rgba8Unorm,
        default());
    let handle = textures.add(image);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(300.0, 300.0)),
                color: Color::rgb(0.5, 0.7, 0.6),
                ..Default::default()
            },
            texture: handle,
            ..Default::default()
        },
        Life::new(),
    ));
}
