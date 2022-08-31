#include "Keyboard.h"
//declaring button pins
const int buttonPin = 3;          
int previousButtonState = HIGH; 
void setup() {
  //declare the buttons as input_pullup
  pinMode(buttonPin, INPUT_PULLUP);  
  Keyboard.begin();
  randomSeed(analogRead(0));
}

void loop() {
  int buttonState = digitalRead(buttonPin);
  //int ran = random(10, 127); // Random button lol
  int ran = 80;
  if (buttonState == LOW && previousButtonState == HIGH) { // Press
    Keyboard.press(ran);
    delay(50);
  }
  if (buttonState == HIGH && previousButtonState == LOW) { // Release
    Keyboard.release(ran);
    delay(50);
  }
  previousButtonState = buttonState;
}