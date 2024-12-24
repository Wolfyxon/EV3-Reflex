use std::{thread, time::{Duration, Instant}};

use ev3dev_lang_rust::{sensors::TouchSensor, sound, Ev3Result, Led};
use rand::Rng;

fn main() -> Ev3Result<()> {
    let mut rng = rand::thread_rng();
    
    let touch = TouchSensor::find().expect("Touch sensor not connected");
    let led = Led::new()?;
    
    led.set_color(Led::COLOR_OFF)?;

    println!("Ready");
    
    loop {
        for _ in 0..rng.gen_range(0..3) {
            led.set_color(Led::COLOR_GREEN)?;
            sleep(0.1, &touch, &led);
            led.set_color(Led::COLOR_OFF)?;
            
            sleep(rng.gen_range(0.1..1.0), &touch, &led);
        }

        sleep(rng.gen_range(0.0..2.0), &touch, &led);

        if rng.gen_bool(0.2) {
            led.set_color(Led::COLOR_RED)?;
            sound::tone(500.0, 10)?;

            let start = Instant::now();
            let duration = Duration::from_secs(2);

            while Instant::now().duration_since(start) < duration {
                if touch.get_pressed_state()? {
                    led.set_color(Led::COLOR_YELLOW)?;
                    
                    if Instant::now().duration_since(start).as_secs_f32() < 0.25 {
                        sound::tone_sequence(&[
                            (500.0, 100, 0),
                            (600.0, 100, 0),
                            (800.0, 100, 0),
                            (1000.0, 200, 0),
                        ])?.wait()?;
                    } else {
                        sound::tone(1000.0, 100)?.wait()?;
                    }

                    break;
                }
            }

            led.set_color(Led::COLOR_OFF)?;
        } else {
            sleep(rng.gen_range(0.0..2.0), &touch, &led);
        }
    }
}

fn check_bad_press(touch: &TouchSensor, led: &Led) {
    if touch.get_pressed_state().unwrap() {
        led.set_color(Led::COLOR_AMBER).unwrap();
        
        sound::tone_sequence(&[
            (800.0, 200, 0),
            (500.0, 200, 0),
            (200.0, 200, 0),
        ]).unwrap().wait().unwrap();

        sleep_unchecked(1.0);

        led.set_color(Led::COLOR_OFF).unwrap()
    }
}

fn sleep(duration: f32, touch: &TouchSensor, led: &Led) {
    let start = Instant::now();
    
    while Instant::now().duration_since(start).as_secs_f32() < duration {
        check_bad_press(&touch, &led);
    }
}

fn sleep_unchecked(duration: f32) {
    thread::sleep(Duration::from_secs_f32(duration));
}