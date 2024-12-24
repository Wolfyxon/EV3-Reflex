use std::{thread, time::Duration};

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
            sleep(0.1);
            led.set_color(Led::COLOR_OFF)?;
            
            sleep(rng.gen_range(0.1..1.0));
        }

        sleep(rng.gen_range(0.0..2.0));

        if rng.gen_bool(0.2) {
            led.set_color(Led::COLOR_RED)?;
            sound::tone(500.0, 10)?;

            while !touch.get_pressed_state()? {}

            led.set_color(Led::COLOR_YELLOW)?;
            sound::tone(1000.0, 200)?.wait()?;
            led.set_color(Led::COLOR_OFF)?;

        } else {
            sleep(rng.gen_range(0.0..2.0));
        }
    }
}

fn sleep(duration: f32) {
    thread::sleep(Duration::from_secs_f32(duration));
}