const int sensorPin = A5;
int sensorValue = 0;

void setup() {
  Serial.begin(9600);
}

void loop() {
  sensorValue = analogRead(sensorPin);
  if (sensorValue > 95) {
    Serial.println(sensorValue);
  }
  // delay(10);
}
