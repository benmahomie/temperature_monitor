import Adafruit_DHT
import json

# Sensor type
DHT_TYPE = Adafruit_DHT.DHT11

# GPIO pin number
GPIO_PIN = 7

humidity, temperature = Adafruit_DHT.read_retry(DHT_TYPE, GPIO_PIN)

if humidity is not None and temperature is not None:
    print(json.dumps({"temperature": temperature, "humidity": humidity}))
else:
    print(json.dumps({"error": "Failed to get reading"}))