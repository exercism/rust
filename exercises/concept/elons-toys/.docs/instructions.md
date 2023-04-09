# Instructions

In this exercise you'll be playing around with a remote controlled car, which you've finally saved enough money for to buy.

Cars start with full (100%) batteries. Each time you drive the car using the remote control, it covers 20 meters and drains one percent of the battery.

The remote controlled car has a fancy LED display that shows two bits of information:

- The total distance it has driven, displayed as: `"Driven <METERS> meters"`.
- The remaining battery charge, displayed as: `"Battery at <PERCENTAGE>%"`.

If the battery is at 0%, you can't drive the car anymore and the battery display will show `"Battery empty"`.

You have six tasks, each of which will work with remote controlled car instances.

## 1. Buy a brand-new remote controlled car

Implement the `RemoteControlCar::buy()` function to return a brand-new remote controlled car instance:

```rust
let car = RemoteControlCar::buy();
```

## 2. Display the distance driven

Implement the `RemoteControlCar::distance_display()` method to return the distance as displayed on the LED display:

```rust
let car = RemoteControlCar::buy();
car.DistanceDisplay();
// => "Driven 0 meters"
```

## 3. Display the battery percentage

Implement the `RemoteControlCar::battery_display()` method to return the battery percentage as displayed on the LED display:

```rust
let car = RemoteControlCar::buy();
car.BatteryDisplay();
// => "Battery at 100%"
```

## 4. Update the number of meters driven when driving

Implement the `RemoteControlCar::drive()` method that updates the number of meters driven:

```rust
let mut car = RemoteControlCar::buy();
car.Drive();
car.Drive();
car.DistanceDisplay();
// => "Driven 40 meters"
```

## 5. Update the battery percentage when driving

Update the `RemoteControlCar::drive()` method to update the battery percentage:

```rust
let mut car = RemoteControlCar::buy();
car.Drive();
car.Drive();
car.BatteryDisplay();
// => "Battery at 98%"
```

## 6. Prevent driving when the battery is drained

Update the `RemoteControlCar::drive()` method to not increase the distance driven nor decrease the battery percentage when the battery is drained (at 0%):

```rust
let car = RemoteControlCar::buy();

// Drain the battery
// ...

car.DistanceDisplay();
// => "Driven 2000 meters"

car.BatteryDisplay();
// => "Battery empty"
```
