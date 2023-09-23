// Import libraries
use rppal::gpio::Gpio;
use std::process::Command;
use serde_json::Value;
use rppal::gpio::Level;
use std::thread;
use std::time::Duration;

fn blink_led(pin: u8, duration: Duration, times: u8) {
    for _ in 0..times {
        set_led_state(pin, Level::High);
        thread::sleep(duration);
        set_led_state(pin, Level::Low);
        thread::sleep(duration);
    }
}

fn set_led_state(pin: u8, state: Level) {
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(pin).unwrap().into_output();
    pin.write(state);
}

// Assume temperature LED is connected to GPIO 18 and humidity LED to GPIO 19
const TEMP_LED_PIN: u8 = 18;
const HUMIDITY_LED_PIN: u8 = 19;

fn read_dht11() -> Result<(f32, f32), String> {
    let output = Command::new("python")
        .arg("read_dht11.py")
        .output()
        .expect("Failed to execute Python script");

    let output_str = String::from_utf8(output.stdout).unwrap();
    let json: Value = serde_json::from_str(&output_str).unwrap();

    if json["error"].is_null() {
        let temperature = json["temperature"].as_f64().unwrap() as f32;
        let humidity = json["humidity"].as_f64().unwrap() as f32;
        Ok((humidity, temperature))
    } else {
        Err(String::from("Failed to get reading"))
    }
}

fn main() {
    // TODO - power on LED if heat range between 15C and 30C
    // TODO - flash warning LED if humidity greater than 50%
    const LOW_TEMP: f32 = 15.0;
    const HIGH_TEMP: f32 = 30.0;
    const HIGH_HUMIDITY: f32 = 50.0;

    // GPIO pins for the LEDs
    const TEMP_LED_PIN: u8 = 18; 
    const HUMIDITY_LED_PIN: u8 = 19; 
    
    loop { 
        match read_dht11() {
            Ok((humidity, temperature)) => {
                println!("Humidity: {}%, Temperature: {}°C", humidity, temperature);

                // Temperature LED logic
                if temperature >= LOW_TEMP && temperature <= HIGH_TEMP {
                    set_led_state(TEMP_LED_PIN, Level::High); // LED on
                } else if temperature < LOW_TEMP {
                    blink_led(TEMP_LED_PIN, Duration::from_millis(1000), 3); // Slowly blink LED
                } else {
                    blink_led(TEMP_LED_PIN, Duration::from_millis(250), 3); // Rapidly blink LED
                }

                // Humidity LED logic
                if humidity > HIGH_HUMIDITY {
                    set_led_state(HUMIDITY_LED_PIN, Level::High); // LED on
                } else {
                    set_led_state(HUMIDITY_LED_PIN, Level::Low); // LED off
                }

            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}