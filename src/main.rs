use std::thread;

use ::rand::*;
// use macroquad::prelude::*;
use road_intersection::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Road intersection".to_owned(),
        window_width: 800,
        window_height: 800,
        fullscreen: false,
        window_resizable: true,
        high_dpi: true,
        sample_count: 1,
        icon: None,
        platform: miniquad::conf::Platform::default(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<Car> = vec![];
    let width = screen_width();
    let height = screen_width();
    let instans_car = vec![
        Car::new(KeyCode::Up, (width / 2., height)),
        Car::new(KeyCode::Down, ((width / 2.) - 50., -40.0)),
        Car::new(KeyCode::Left, (height, (width / 2.) - 50.)),
        Car::new(KeyCode::Right, (-40., width / 2.)),
    ];
    'my_loop: loop {
        clear_background(BLACK);

        // borders
        draw_line_ori(width, height);
        draw_line_ori(width, height + 100.);
        draw_line_ori(width, height - 100.);
        draw_line_ver(width, height);
        draw_line_ver(width + 100., height);
        draw_line_ver(width - 100., height);

        // cars
        for ele in &mut cars {
            draw_rectangle(ele.pos.0, ele.pos.1, 50., 50., ele.color);
            match ele.dir {
                KeyCode::Up => ele.pos.1 -= 2.,
                KeyCode::Down => ele.pos.1 += 2.,
                KeyCode::Left => ele.pos.0 -= 2.,
                _ => ele.pos.0 += 2.,
            }
        }

        for key in get_keys_pressed() {
            match key {
                KeyCode::Escape => {
                    break 'my_loop;
                }
                KeyCode::Up => {
                    let mut car = instans_car[0].clone();
                    car.color = Car::random_color();
                    cars.push(car);
                    
                }
                KeyCode::Down => {
                    let mut car = instans_car[1].clone();
                    car.color = Car::random_color();
                    cars.push(car);
                }
                KeyCode::Left => {
                    let mut car = instans_car[2].clone();
                    car.color = Car::random_color();
                    cars.push(car);
                }
                KeyCode::Right => {
                    let mut car = instans_car[3].clone();
                    car.color = Car::random_color();
                    cars.push(car);
                }
                KeyCode::R => {
                    let mut car = instans_car[random_range(0..4)].clone();
                    car.color = Car::random_color();
                    cars.push(car);
                }
                _ => {}
            }
        }

        next_frame().await
    }
}

fn draw_line_ori(width: f32, hight: f32) {
    draw_line(0., hight / 2., width, hight / 2., 1., WHITE);
}

fn draw_line_ver(width: f32, hight: f32) {
    draw_line(width / 2., 0., width / 2., hight, 1., WHITE);
}

