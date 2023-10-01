# Overview

As a software engineer with an interest in embedded systems, I wanted to see how I could use Rust to create a simple temperature monitor.

The software is a Rust program I designed to control LEDs based on temperature and humidity readings. It reads these values from a DHT11 sensor through a Python script.

The LEDs indicate:

#### Temperature LED
- **On**: If temperature is between 15°C and 30°C.
- **Slow Blink**: If temperature is below 15°C.
- **Fast Blink**: If temperature is above 30°C.

#### Humidity LED
- **On**: If humidity is above 50%.

I created this software primarily to learn a little more about the Rust language, but also to create a tool that could help me visually in remotely monitoring room conditions for my 3d printer. 
The temperature ranges and humidity threshold are recommended thresholds for PLA filament. If the filament is outside these thresholds, it can become damaged.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

Both the Rust code and Python helper were made using the VSCode IDE. The Rust code could not be tested in my development environment, so it was transferred to the Pi Zero W via SCP and then run from the command line via SSH.
SCP and SSH connections were done through connection to a personal 2.4GHZ hotspot.

Languges used were Rust, used in src/main.rs, and Python, used as a translator tool for the DHT11 sensor module.

# Useful Websites

These websites helped me understand how to connect my DHT11 to my Raspberry Pi Zero W. I learned more about the capabilities of the Rust language from ChatGPT and the Rust Playground.

- [Pi Zero W Pinout](https://kalitut.com/raspberry-pi-zero-w-review/)
- [DHT11 Pinout](https://www.circuitbasics.com/how-to-set-up-the-dht11-humidity-sensor-on-an-arduino/)
- [Chat GPT](https://chat.openai.com/)
- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

# Future Work

The following still need to be improved:

- LEDs are too weak to see from a monitoring camera
- Python helper script could get a Rust replacement with lots of work and info
