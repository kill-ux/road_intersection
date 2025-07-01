use std::{
    thread,
    time::{Duration, Instant},
};

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
        Car::new(KeyCode::Down, ((width / 2.) - 50., -50.0)),
        Car::new(KeyCode::Left, (height, (width / 2.) - 50.)),
        Car::new(KeyCode::Right, (-50., width / 2.)),
    ];

    let mut last_green = (Instant::now(), 0);

    let mut lights = [
        Lights {
            pos: (450., 450.),
            color: RED,
        },
        Lights {
            pos: (300., 300.),
            color: RED,
        },
        Lights {
            pos: (300., 450.),
            color: RED,
        },
        Lights {
            pos: (450., 300.),
            color: RED,
        },
    ];

    let mut stack_down: Option<Car> = None;
    let mut stack_up: Option<Car> = None;
    let mut stack_left: Option<Car> = None;
    let mut stack_right: Option<Car> = None;

    let mut cool_down_up = Instant::now();
    let mut cool_down_down = Instant::now();
    let mut cool_down_left = Instant::now();
    let mut cool_down_right = Instant::now();
    let mut cool_down_r = Instant::now();

    'my_loop: loop {
        clear_background(BLACK);

        // borders
        draw_line_ori(width, height);
        draw_line_ori(width, height + 100.);
        draw_line_ori(width, height - 100.);
        draw_line_ver(width, height);
        draw_line_ver(width + 100., height);
        draw_line_ver(width - 100., height);

        if last_green.0.elapsed() > Duration::from_secs(2) {
            lights[last_green.1].color = RED;
            last_green.0 = Instant::now();
            last_green.1 += 1;
            if last_green.1 == 4 {
                last_green.1 = 0;
            }
            lights[last_green.1].color = GREEN;
            match last_green.1 {
                0 => stack_up = None,
                1 => stack_down = None,
                3 => stack_left = None,
                4 => stack_right = None,
                _ => {}
            }
        }

        //ligths
        for ele in &lights {
            draw_rectangle_lines(ele.pos.0, ele.pos.1, 50., 50., 2., ele.color);
        }

        // cars
        for ele in &mut cars {
            draw_rectangle(ele.pos.0, ele.pos.1, 50., 50., ele.color);

            match ele.dir {
                KeyCode::Up => {
                    if lights[0].color != RED
                        || lights[0].color == RED && !(ele.pos.1 == 450. && ele.pos.0 == 400.)
                    {
                        if stack_up.is_none() || stack_up.clone().unwrap().pos.1 + 60. < ele.pos.1 {
                            ele.pos.1 -= 2.;
                            match ele.color {
                                GREEN if ele.pos.1 == 400. && !ele.is_moved => {
                                    ele.dir = KeyCode::Right;
                                    ele.is_moved = true;
                                }
                                YELLOW if ele.pos.1 == 350. && !ele.is_moved => {
                                    ele.dir = KeyCode::Left;
                                    ele.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_up = Some(ele.clone())
                        }
                    } else {
                        if stack_up.is_none() {
                            stack_up = Some(ele.clone())
                        }
                    }
                }
                KeyCode::Down => {
                    if lights[1].color != RED
                        || lights[1].color == RED && !(ele.pos.1 == 300. && ele.pos.0 == 350.)
                    {
                        if stack_down.is_none()
                            || stack_down.clone().unwrap().pos.1 > ele.pos.1 + 60.
                        {
                            ele.pos.1 += 2.;
                            match ele.color {
                                GREEN if ele.pos.1 == 350. && !ele.is_moved => {
                                    ele.dir = KeyCode::Left;
                                    ele.is_moved = true;
                                }
                                YELLOW if ele.pos.1 == 400. && !ele.is_moved => {
                                    ele.dir = KeyCode::Right;
                                    ele.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_down = Some(ele.clone());
                        }
                    } else {
                        if stack_down.is_none() {
                            stack_down = Some(ele.clone());
                        }
                    }
                }
                KeyCode::Left => {
                    if lights[3].color != RED
                        || lights[3].color == RED && !(ele.pos.0 == 450. && ele.pos.1 == 350.)
                    {
                        if stack_left.is_none()
                            || stack_left.clone().unwrap().pos.0 + 60. < ele.pos.0
                        {
                            ele.pos.0 -= 2.;
                            match ele.color {
                                YELLOW if ele.pos.0 == 350. && !ele.is_moved => {
                                    ele.dir = KeyCode::Down;
                                    ele.is_moved = true;
                                }
                                GREEN if ele.pos.0 == 400. && !ele.is_moved => {
                                    ele.dir = KeyCode::Up;
                                    ele.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_left = Some(ele.clone());
                        }
                    } else {
                        if stack_left.is_none() {
                            stack_left = Some(ele.clone());
                        }
                    }
                }
                KeyCode::Right => {
                    if lights[2].color != RED
                        || lights[2].color == RED && ele.pos.0 != 300. && ele.pos.1 != 450.
                    {
                        if stack_right.is_none()
                            || stack_right.clone().unwrap().pos.0 > ele.pos.0 + 60.
                        {
                            ele.pos.0 += 2.;
                            match ele.color {
                                GREEN if ele.pos.0 == 350. && !ele.is_moved => {
                                    ele.dir = KeyCode::Down;
                                    ele.is_moved = true;
                                }
                                YELLOW if ele.pos.0 == 400. && !ele.is_moved => {
                                    ele.dir = KeyCode::Up;
                                    ele.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_right = Some(ele.clone());
                        }
                    } else {
                        if stack_right.is_none() {
                            stack_right = Some(ele.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        for key in get_keys_pressed() {
            match key {
                KeyCode::Escape => {
                    break 'my_loop;
                }
                KeyCode::Up => {
                    if cool_down_up.elapsed() > Duration::from_secs_f32(0.6) {
                        let mut car = instans_car[0].clone();
                        car.color = Car::random_color();
                        cars.push(car);
                        cool_down_up = Instant::now();
                    }
                }
                KeyCode::Down => {
                    if cool_down_down.elapsed() > Duration::from_secs_f32(0.8) {
                        let mut car = instans_car[1].clone();
                        car.color = Car::random_color();
                        cars.push(car);
                        cool_down_down = Instant::now();
                    }
                }
                KeyCode::Left => {
                    if cool_down_left.elapsed() > Duration::from_secs_f32(0.8) {
                        let mut car = instans_car[2].clone();
                        car.color = Car::random_color();
                        cars.push(car);
                        cool_down_left = Instant::now();
                    }
                }
                KeyCode::Right => {
                    if cool_down_right.elapsed() > Duration::from_secs_f32(0.8) {
                        let mut car = instans_car[3].clone();
                        car.color = Car::random_color();
                        cars.push(car);
                        cool_down_right = Instant::now();
                    }
                }
                KeyCode::R => {
                    if cool_down_r.elapsed() > Duration::from_secs_f32(0.8) {
                        let mut car = instans_car[random_range(0..4)].clone();
                        car.color = Car::random_color();
                        cars.push(car);
                        cool_down_r = Instant::now()
                    }
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
