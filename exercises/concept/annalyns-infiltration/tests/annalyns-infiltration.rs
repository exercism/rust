#[test]
fn cannot_execute_fast_attack_if_knight_is_awake() {
    let knight_is_awake = true;
    assert!(!annalyns_infiltration::can_fast_attack(knight_is_awake));
}

#[test]
#[ignore]
fn can_execute_fast_attack_if_knight_is_sleeping() {
    let knight_is_awake = false;
    assert!(annalyns_infiltration::can_fast_attack(knight_is_awake));
}

#[test]
#[ignore]
fn cannot_spy_if_everyone_is_sleeping() {
    let knight_is_awake = false;
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    assert!(!annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_everyone_but_knight_is_sleeping() {
    let knight_is_awake = true;
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_everyone_but_archer_is_sleeping() {
    let knight_is_awake = false;
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_everyone_but_prisoner_is_sleeping() {
    let knight_is_awake = false;
    let archer_is_awake = false;

    let prisoner_is_awake = true;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_only_knight_is_sleeping() {
    let knight_is_awake = false;
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_only_archer_is_sleeping() {
    let knight_is_awake = true;
    let archer_is_awake = false;
    let prisoner_is_awake = true;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_only_prisoner_is_sleeping() {
    let knight_is_awake = true;
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_spy_if_everyone_is_awake() {
    let knight_is_awake = true;
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    assert!(annalyns_infiltration::can_spy(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn can_signal_prisoner_if_archer_is_sleeping_and_prisoner_is_awake() {
    let archer_is_awake = false;
    let prisoner_is_awake = true;
    assert!(annalyns_infiltration::can_signal_prisoner(
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn cannot_signal_prisoner_if_archer_is_awake_and_prisoner_is_sleeping() {
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    assert!(!annalyns_infiltration::can_signal_prisoner(
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn cannot_signal_prisoner_if_archer_and_prisoner_are_both_sleeping() {
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    assert!(!annalyns_infiltration::can_signal_prisoner(
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn cannot_signal_prisoner_if_archer_and_prisoner_are_both_awake() {
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    assert!(!annalyns_infiltration::can_signal_prisoner(
        archer_is_awake,
        prisoner_is_awake
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_everyone_is_awake_and_pet_dog_is_present() {
    let knight_is_awake = true;
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    let pet_dog_is_present = true;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_everyone_is_awake_and_pet_dog_is_absent() {
    let knight_is_awake = true;
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn can_free_prisoner_if_everyone_is_asleep_and_pet_dog_is_present() {
    let knight_is_awake = false;
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    let pet_dog_is_present = true;
    assert!(annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_everyone_is_asleep_and_pet_dog_is_absent() {
    let knight_is_awake = false;
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn can_free_prisoner_if_only_prisoner_is_awake_and_pet_dog_is_present() {
    let knight_is_awake = false;
    let archer_is_awake = false;
    let prisoner_is_awake = true;
    let pet_dog_is_present = true;
    assert!(annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn can_free_prisoner_if_only_prisoner_is_awake_and_pet_dog_is_absent() {
    let knight_is_awake = false;
    let archer_is_awake = false;
    let prisoner_is_awake = true;
    let pet_dog_is_present = false;
    assert!(annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_archer_is_awake_and_pet_dog_is_present() {
    let knight_is_awake = false;
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    let pet_dog_is_present = true;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_archer_is_awake_and_pet_dog_is_absent() {
    let knight_is_awake = false;
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn can_free_prisoner_if_only_knight_is_awake_and_pet_dog_is_present() {
    let knight_is_awake = true;
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    let pet_dog_is_present = true;
    assert!(annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_knight_is_awake_and_pet_dog_is_absent() {
    let knight_is_awake = true;
    let archer_is_awake = false;
    let prisoner_is_awake = false;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_knight_is_asleep_and_pet_dog_is_present() {
    let knight_is_awake = false;
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    let pet_dog_is_present = true;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_knight_is_asleep_and_pet_dog_is_absent() {
    let knight_is_awake = false;
    let archer_is_awake = true;
    let prisoner_is_awake = true;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn can_free_prisoner_if_only_archer_is_asleep_and_pet_dog_is_present() {
    let knight_is_awake = true;
    let archer_is_awake = false;
    let prisoner_is_awake = true;
    let pet_dog_is_present = true;
    assert!(annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_archer_is_asleep_and_pet_dog_is_absent() {
    let knight_is_awake = true;
    let archer_is_awake = false;
    let prisoner_is_awake = true;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_prisoner_is_asleep_and_pet_dog_is_present() {
    let knight_is_awake = true;
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    let pet_dog_is_present = true;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}

#[test]
#[ignore]
fn cannot_free_prisoner_if_only_prisoner_is_asleep_and_pet_dog_is_absent() {
    let knight_is_awake = true;
    let archer_is_awake = true;
    let prisoner_is_awake = false;
    let pet_dog_is_present = false;
    assert!(!annalyns_infiltration::can_free_prisoner(
        knight_is_awake,
        archer_is_awake,
        prisoner_is_awake,
        pet_dog_is_present
    ));
}
