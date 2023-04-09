use elons_toys::*;

#[test]
pub fn new_car_distance_display() {
    let car = RemoteControlCar::buy();
    assert_eq!("Driven 0 meters", car.distance_display());
}

#[test]
#[ignore]
pub fn new_car_battery_display() {
    let car = RemoteControlCar::buy();
    assert_eq!("Battery at 100%", car.battery_display());
}

#[test]
#[ignore]
pub fn distance_display_after_driving_once() {
    let mut car = RemoteControlCar::buy();
    car.drive();
    assert_eq!("Driven 20 meters", car.distance_display());
}

#[test]
#[ignore]
pub fn distance_display_after_driving_multiple_times() {
    let mut car = RemoteControlCar::buy();

    for _ in 1..=17 {
        car.drive();
    }

    assert_eq!("Driven 340 meters", car.distance_display());
}

#[test]
#[ignore]
pub fn battery_display_after_driving_once() {
    let mut car = RemoteControlCar::buy();
    car.drive();
    assert_eq!("Battery at 99%", car.battery_display());
}

#[test]
#[ignore]
pub fn battery_display_after_driving_multiple_times() {
    let mut car = RemoteControlCar::buy();

    for _ in 1..=23 {
        car.drive();
    }

    assert_eq!("Battery at 77%", car.battery_display());
}

#[test]
#[ignore]
pub fn battery_display_when_battery_empty() {
    let mut car = RemoteControlCar::buy();

    // Drain the battery
    for _ in 1..=100 {
        car.drive();
    }

    // Attempt to drive one more time (should not work)
    car.drive();

    assert_eq!("Battery empty", car.battery_display());
}

#[test]
#[ignore]
pub fn distance_display_when_battery_empty() {
    let mut car = RemoteControlCar::buy();

    // Drain the battery
    for _ in 1..=100 {
        car.drive();
    }

    // Attempt to drive one more time (should not work)
    car.drive();

    assert_eq!("Driven 2000 meters", car.distance_display());
}
