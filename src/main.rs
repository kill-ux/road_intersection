use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

use ::rand::*;
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

fn draw_roads() {
    let center = 400.0;
    let lane_width = 60.0;

    // Compute positions based on center and lane width
    let left = center - lane_width;
    let right = center + lane_width;
    let top = center - lane_width;
    let bottom = center + lane_width;

    let road_lines = [
        // Horizontal roads
        (0.0, top, left, top, LIGHTGRAY),          // East top
        (0.0, center, left, center, DARKGRAY),     // East middle
        (0.0, bottom, left, bottom, LIGHTGRAY),    // East bottom
        (right, top, 800.0, top, LIGHTGRAY),       // West top
        (right, center, 800.0, center, DARKGRAY),  // West middle
        (right, bottom, 800.0, bottom, LIGHTGRAY), // West bottom
        // Vertical roads
        (top, 0.0, top, left, LIGHTGRAY),          // North left
        (center, 0.0, center, left, DARKGRAY),     // North middle
        (bottom, 0.0, bottom, left, LIGHTGRAY),    // North right
        (top, right, top, 800.0, LIGHTGRAY),       // South left
        (center, right, center, 800.0, DARKGRAY),  // South middle
        (bottom, right, bottom, 800.0, LIGHTGRAY), // South right
    ];

    for (x1, y1, x2, y2, color) in road_lines {
        draw_line(x1, y1, x2, y2, 1.0, color);
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<RefCell<Car>> = vec![];
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
            pos: (460., 460.),
            color: GREEN,
        },
        Lights {
            pos: (340., 340.),
            color: RED,
        },
        Lights {
            pos: (340., 460.),
            color: RED,
        },
        Lights {
            pos: (460., 340.),
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

    let mut time_up = (false, Instant::now());
    let mut time_down = (false, Instant::now());
    let mut time_left: (bool, Instant) = (false, Instant::now());
    let mut time_right = (false, Instant::now());

    'my_loop: loop {
        clear_background(BLACK);

        // Draw borders and lights (unchanged)
        // draw_line_ori(width, height);
        // draw_line_ori(width, height);
        // draw_line(0., height / 2., width, height / 2., 1., WHITE);

        // draw_line_ori(width, height + 100.);
        // draw_line_ori(width, height - 100.);
        // draw_line_ver(width, height);
        // draw_line_ver(width + 100., height);
        // draw_line_ver(width - 100., height);
        draw_roads();

        if last_green.0.elapsed() > Duration::from_secs(5) {
            lights[last_green.1].color = RED;
            last_green.0 = Instant::now();

            last_green.1 = (last_green.1 + 1) % 4;
            lights[last_green.1].color = GREEN;
            match last_green.1 {
                0 => stack_up = None,
                1 => stack_down = None,
                2 => stack_right = None,
                3 => stack_left = None,
                _ => {}
            }
        }

        for ele in &lights {
            draw_circle_lines(ele.pos.0, ele.pos.1, 8., 5., ele.color);
            draw_circle(ele.pos.0, ele.pos.1, 8., BLACK);
        }

        // Process cars
        for car_cell in &cars {
            let mut car = car_cell.borrow_mut();
            // draw_rectangle(car.pos.0, car.pos.1, 50., 50., BLACK);
            draw_rectangle(car.pos.0 + 10., car.pos.1 + 10., 30., 30., car.color);

            //

            if (lights[0].color == GREEN && car.pos.1 == 450. && car.pos.0 == 400.)
                || (lights[1].color == GREEN && car.pos.1 == 300. && car.pos.0 == 350.)
                || (lights[3].color == GREEN && car.pos.0 == 450. && car.pos.1 == 350.)
                || (lights[2].color == GREEN && car.pos.0 == 300. && car.pos.1 == 400.)
            {
                let mut stop: bool = false;
                for car_cell2 in &cars {
                    if std::ptr::eq(car_cell, car_cell2) {
                        continue; // Skip the same car
                    }
                    let car2 = car_cell2.borrow();
                    if car2.pos.0 > 300.
                        && car2.pos.0 < 450.
                        && car2.pos.1 > 300.
                        && car2.pos.1 < 450.
                    {
                        stop = true;
                    }
                }
                if stop {
                    match car.dir {
                        KeyCode::Up => {
                            stack_up = {
                                time_up = (true, Instant::now());
                                Some(car.clone())
                            }
                        }
                        KeyCode::Down => {
                            stack_down = {
                                time_down = (true, Instant::now());
                                Some(car.clone())
                            }
                        }
                        KeyCode::Left => {
                            stack_left = {
                                time_left = (true, Instant::now());
                                Some(car.clone())
                            }
                        }
                        KeyCode::Right => {
                            stack_right = {
                                time_right = (true, Instant::now());
                                Some(car.clone())
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
            }

            if time_up.1.elapsed() > Duration::from_secs_f32(0.7) && time_up.0 {
                stack_up = None;
                time_up = (false, Instant::now());
            }

            if time_down.1.elapsed() > Duration::from_secs_f32(0.7) && time_down.0 {
                stack_down = None;

                time_down = (false, Instant::now());
            }

            if time_left.1.elapsed() > Duration::from_secs_f32(0.7) && time_left.0 {
                stack_left = None;
                time_left = (false, Instant::now());
            }

            if time_right.1.elapsed() > Duration::from_secs_f32(0.7) && time_right.0 {
                stack_right = None;
                time_right = (false, Instant::now());
            }

            match car.dir {
                KeyCode::Up => {
                    if lights[0].color != RED
                        || (lights[0].color == RED && !(car.pos.1 == 450. && car.pos.0 == 400.))
                    {
                        if stack_up.is_none()
                            || car.pos.1 < 450.
                            || stack_up.as_ref().unwrap().pos.1 + 60. < car.pos.1
                        {
                            car.pos.1 -= 2.;
                            match car.color {
                                GREEN if car.pos.1 == 400. && !car.is_moved => {
                                    car.dir = KeyCode::Right;
                                    car.is_moved = true;
                                }
                                YELLOW if car.pos.1 == 350. && !car.is_moved => {
                                    car.dir = KeyCode::Left;
                                    car.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_up = Some(car.clone());
                        }
                    } else {
                        stack_up = Some(car.clone());
                    }
                }
                KeyCode::Down => {
                    if lights[1].color != RED
                        || (lights[1].color == RED && !(car.pos.1 == 300. && car.pos.0 == 350.))
                    {
                        if stack_down.is_none()
                            || car.pos.1 > 300.
                            || stack_down.as_ref().unwrap().pos.1 > car.pos.1 + 60.
                        {
                            car.pos.1 += 2.;
                            match car.color {
                                GREEN if car.pos.1 == 350. && !car.is_moved => {
                                    car.dir = KeyCode::Left;
                                    car.is_moved = true;
                                }
                                YELLOW if car.pos.1 == 400. && !car.is_moved => {
                                    car.dir = KeyCode::Right;
                                    car.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_down = Some(car.clone());
                        }
                    } else {
                        stack_down = Some(car.clone());
                    }
                }
                KeyCode::Left => {
                    if lights[3].color != RED
                        || (lights[3].color == RED && !(car.pos.0 == 450. && car.pos.1 == 350.))
                    {
                        if stack_left.is_none()
                            || car.pos.0 < 450.
                            || stack_left.as_ref().unwrap().pos.0 + 60. < car.pos.0
                        {
                            car.pos.0 -= 2.;
                            match car.color {
                                YELLOW if car.pos.0 == 350. && !car.is_moved => {
                                    car.dir = KeyCode::Down;
                                    car.is_moved = true;
                                }
                                GREEN if car.pos.0 == 400. && !car.is_moved => {
                                    car.dir = KeyCode::Up;
                                    car.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_left = Some(car.clone());
                        }
                    } else {
                        stack_left = Some(car.clone());
                    }
                }
                KeyCode::Right => {
                    if lights[2].color != RED
                        || (lights[2].color == RED && car.pos.0 != 300. && car.pos.1 != 450.)
                    {
                        if stack_right.is_none()
                            || car.pos.0 > 300.
                            || stack_right.as_ref().unwrap().pos.0 > car.pos.0 + 60.
                        {
                            car.pos.0 += 2.;
                            match car.color {
                                GREEN if car.pos.0 == 350. && !car.is_moved => {
                                    car.dir = KeyCode::Down;
                                    car.is_moved = true;
                                }
                                YELLOW if car.pos.0 == 400. && !car.is_moved => {
                                    car.dir = KeyCode::Up;
                                    car.is_moved = true;
                                }
                                _ => {}
                            };
                        } else {
                            stack_right = Some(car.clone());
                        }
                    } else {
                        stack_right = Some(car.clone());
                    }
                }
                _ => {}
            }
        }

        

          // Remove cars that have gone off screen
        cars.retain(|car_cel| {
            if let Ok(car) = car_cel.try_borrow() {
                car.pos.0 > -100. && car.pos.0 < width + 100. && 
                car.pos.1 > -100. && car.pos.1 < height + 100.
            } else {
                true
            }
        });

        // Key handling (unchanged)
        let mut match_keys = |key: KeyCode| {
            match key {
                KeyCode::Up => {
                    if cool_down_up.elapsed() > Duration::from_secs_f32(0.6)
                        && (stack_up.is_none()
                            || stack_up
                                .as_ref()
                                .is_some_and(|car| car.pos.1 + 60. < height))
                    {
                        let mut car = instans_car[0].clone();
                        car.color = Car::random_color();
                        cars.push(RefCell::new(car));
                        cool_down_up = Instant::now();
                    }
                }
                KeyCode::Down => {
                    if cool_down_down.elapsed() > Duration::from_secs_f32(0.6)
                        && (stack_down.is_none()
                            || stack_down.as_ref().is_some_and(|car| car.pos.1 > 60.))
                    {
                        let mut car = instans_car[1].clone();
                        car.color = Car::random_color();
                        cars.push(RefCell::new(car));
                        cool_down_down = Instant::now();
                    }
                }
                KeyCode::Left => {
                    if cool_down_left.elapsed() > Duration::from_secs_f32(0.6)
                        && (stack_left.is_none()
                            || stack_left
                                .as_ref()
                                .is_some_and(|car| car.pos.0 + 60. < width))
                    {
                        let mut car = instans_car[2].clone();
                        car.color = Car::random_color();
                        cars.push(RefCell::new(car));
                        cool_down_left = Instant::now();
                    }
                }
                KeyCode::Right => {
                    if cool_down_right.elapsed() > Duration::from_secs_f32(0.6)
                        && (stack_right.is_none()
                            || stack_right.as_ref().is_some_and(|car| car.pos.0 > 60.))
                    {
                        let mut car = instans_car[3].clone();
                        car.color = Car::random_color();
                        cars.push(RefCell::new(car));
                        cool_down_right = Instant::now();
                    }
                }
                _ => {}
            };
        };

        for key in get_keys_pressed() {
            match key {
                KeyCode::Escape => break 'my_loop,
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => match_keys(key),
                KeyCode::R => {
                    let key_r = [KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]
                        [random_range(0..4)];
                    match_keys(key_r);
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
