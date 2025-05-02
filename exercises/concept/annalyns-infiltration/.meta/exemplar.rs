pub fn can_fast_attack(knight_is_awake: bool) -> bool {
    !knight_is_awake
}

pub fn can_spy(knight_is_awake: bool, archer_is_awake: bool, prisoner_is_awake: bool) -> bool {
    knight_is_awake || archer_is_awake || prisoner_is_awake
}

pub fn can_signal_prisoner(archer_is_awake: bool, prisoner_is_awake: bool) -> bool {
    !archer_is_awake && prisoner_is_awake
}

pub fn can_free_prisoner(
    knight_is_awake: bool,
    archer_is_awake: bool,
    prisoner_is_awake: bool,
    pet_dog_is_present: bool,
) -> bool {
    !knight_is_awake && !archer_is_awake && prisoner_is_awake
        || !archer_is_awake && pet_dog_is_present
}
