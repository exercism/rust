pub struct RemoteControlCar {
    battery_percentage: i32,        //100
    distance_driven_in_meters: i32, // 0
}

impl RemoteControlCar {
    pub fn drive(&mut self) {
        if self.battery_percentage > 0 {
            self.battery_percentage -= 1;
            self.distance_driven_in_meters += 20;
        }
    }

    pub fn distance_display(&self) -> String {
        return format!("Driven {} meters", self.distance_driven_in_meters);
    }

    pub fn battery_display(&self) -> String {
        if self.battery_percentage == 0 {
            return "Battery empty".to_string();
        }

        return format!("Battery at {}%", self.battery_percentage);
    }

    pub fn buy() -> Self {
        return Self {
            battery_percentage: 100,
            distance_driven_in_meters: 0,
        };
    }
}
