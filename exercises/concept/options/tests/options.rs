use options::*;

#[test]
fn test_reviving_dead_player() {
    let level: u32 = 34;
    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level,
    };
    let revived_player = dead_player.revive().expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, Some(100));
    assert_eq!(revived_player.level, level);
}

#[test]
#[ignore]
fn test_reviving_alive_player() {
    let level: u32 = 8;
    let alive_player = Player {
        health: 1,
        mana: None,
        level,
    };
    assert_eq!(alive_player.revive().is_none(), true);
}

#[test]
#[ignore]
fn test_cast_spell_with_enough_mana() {
    let health = 99;
    let mana = 100;
    let level = 100;
    let mana_cost = 3;

    let mut accomplished_wizard = Player {
        health,
        mana: Some(mana),
        level,
    };

    assert_eq!(accomplished_wizard.cast_spell(mana_cost), mana_cost * 2);
    assert_eq!(accomplished_wizard.health, health);
    assert_eq!(accomplished_wizard.mana, Some(mana - mana_cost));
    assert_eq!(accomplished_wizard.level, level);
}

#[test]
#[ignore]
fn test_cast_spell_with_insufficient_mana() {
    let health = 56;
    let mana = 0;
    let level = 22;
    let mana_cost = 3;

    let mut no_mana_wizard = Player {
        health,
        mana: Some(mana),
        level,
    };

    assert_eq!(no_mana_wizard.cast_spell(mana_cost), 0);
    assert_eq!(no_mana_wizard.health, health);
    assert_eq!(no_mana_wizard.mana, Some(mana));
    assert_eq!(no_mana_wizard.level, level);
}

#[test]
#[ignore]
fn test_cast_spell_with_no_mana_pool() {
    let health = 87;
    let level = 6;
    let mana_cost = 3;

    let mut underleveled_player = Player {
        health,
        mana: None,
        level,
    };

    assert_eq!(underleveled_player.cast_spell(mana_cost), 0);
    assert_eq!(underleveled_player.health, health);
    assert_eq!(underleveled_player.mana, None);
    assert_eq!(underleveled_player.level, level);
}
