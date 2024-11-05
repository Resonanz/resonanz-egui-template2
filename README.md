# Resonanz egui Template #2

- Clone ```git clone https://github.com/Resonanz/resonanz-egui-template2.git```
- cd into folder ```cd resonanz-egui-template2```
- Run ```cargo run``` or ```cargo run --release```  ```// useful to compare debug vs release```

## Tests

### CPU i7-5600U, 8 GB ram, CPU load measurements using btop on Ubuntu

The 7-segment font counters are updating flat out with every screen redraw (probably 60 Hz).

* 600 spinners: Window minimized, CPU = 0.3-0.6%
* 1 spinners: Window maximized, CPU = 17-18%, this high CPU load will be due to the 7-segments updating 
* 600 spinners: Window maximized, CPU = 17-19%, 7-segments updating 

## Notes
- Fonts are from fonts.google.com.

![Screenshot from 2024-10-21 16-17-17](https://github.com/user-attachments/assets/d7ce3ca2-3f26-4a1a-bb67-6dd7b762d092)
