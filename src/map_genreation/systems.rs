use bevy::math::Vec2;
use bevy::reflect::Array;
use noise::{NoiseFn, Perlin, Worley};

pub fn default_system(){}

pub fn worley_system(){
    let worley = Perlin::new(1203);
    let mut values: [char;10000] = ['0';10000];
    for i in 0..100 {
        for j in 0..100 {
            let noise = worley.get([i as f64 / 10.0,j as f64 /10.0]);
            match noise {
                ..-0.5 => values[i*100+j] = '#',
                ..0.0  => values[i*100+j] = 'O',
                ..0.5  => values[i*100+j] = 'o',
                _ => values[i*100+j] = '-',
            }
        }
    }
    for i in 0..100 {
        for j in 0..100 {
            print!("{}", values[i*100+j]);
        }
        print!("\n")
    }
}