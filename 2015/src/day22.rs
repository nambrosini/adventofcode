use itertools::Itertools;
use std::cmp;

#[aoc_generator(day22)]
fn generator(input: &str) -> Vec<i16> {
    let mut input = input.lines().map(|s| s[s.len() - 2..].parse().unwrap()).collect_vec();
    input.push(0);
    input
}

#[aoc(day22, part1)]
fn part1(input: &[i16]) -> i16 {
    // 0: heath, 1: mana, 2: armor, 3: turns shield is active, and 4: turns recharge is active
    let player_start: Vec<i16> = vec![50, 500, 0, 0, 0];

    // 0: health, 1: damage, 2: turns poison is active, 
    let boss_start: Vec<i16> = input.to_vec();

    let mut queue: Vec<(Vec<i16>, Vec<i16>, i16, bool)> = Vec::new();
    queue.push((player_start, boss_start, 0, true));

    let mut least_mana_used = 10000;

    while queue.is_empty() == false {
        let queue_entry = queue.pop().unwrap();
        let mut player = queue_entry.0;
        let mut boss = queue_entry.1;
        let mana_used = queue_entry.2;
        let players_turn = queue_entry.3;

        if mana_used >= least_mana_used {
            continue;
        }

        /////////// first handle the effects
        // handle shield effect
        if player[3] > 0 {
            player[2] = 7;
            player[3] -= 1;
        } else {
            player[2] = 0;
        }

        // handle recharge effect
        if player[4] > 0 {
            player[1] += 101;
            player[4] -= 1;
        }

        // handle poison effect
        if boss[2] > 0 {
            boss[0] -= 3;
            boss[2] -= 1;

            if boss[0] <= 0 {
                if mana_used < least_mana_used {
                    least_mana_used = mana_used;
                }

                continue;
            }
        }

        if players_turn {
            // cast magic missile
            if player[1] >= 53 {
                let mut player_copy = player.clone();
                let mut boss_copy = boss.clone();

                boss_copy[0] -= 4;

                let new_mana_used = mana_used + 53;
                if boss_copy[0] <= 0 {
                    if new_mana_used < least_mana_used {
                        least_mana_used = new_mana_used;
                    }
                } else {
                    player_copy[1] -= 53;
                    queue.push((player_copy, boss_copy, new_mana_used, false));
                }
            }
            
            // cast drain
            if player[1] >= 73 {
                let mut player_copy = player.clone();
                let mut boss_copy = boss.clone();

                player_copy[0] += 2;
                boss_copy[0] -= 2;
                
                let new_mana_used = mana_used + 73;
                if boss_copy[0] <= 0 {
                    if new_mana_used < least_mana_used {
                        least_mana_used = new_mana_used;
                    }
                } else {
                    player_copy[1] -= 73;
                    queue.push((player_copy, boss_copy, new_mana_used, false));
                }
            }
            
            // cast shield
            if player[1] >= 113 && player[3] == 0 {
                let mut player_copy = player.clone();

                player_copy[1] -= 113;
                player_copy[3] = 6;
                
                queue.push((player_copy, boss.clone(), mana_used + 113, false));
            }
            
            // cast recharge
            if player[1] >= 229 && player[4] == 0 {
                let mut player_copy = player.clone();

                player_copy[1] -= 229;
                player_copy[4] = 5;

                queue.push((player_copy, boss.clone(), mana_used + 229, false));
            }

            // cast poison
            if player[1] >= 173 && boss[2] == 0{
                let mut player_copy = player.clone();
                let mut boss_copy = boss.clone();

                player_copy[1] -= 173;
                boss_copy[2] = 6;

                queue.push((player_copy, boss_copy, mana_used + 173, false));
            }

        } else {
            player[0] -= cmp::max(1, boss[1] - player[2]);
            if player[0] <= 0 {
                continue;
            }

            queue.push((player.clone(), boss.clone(), mana_used, true));
        }
    }

    least_mana_used
}

#[aoc(day22, part2)]
fn part2(input: &[i16]) -> i16 {
    // 0: heath, 1: mana, 2: armor, 3: turns shield is active, and 4: turns recharge is active
    let player_start: Vec<i16> = vec![50, 500, 0, 0, 0];

    // 0: health, 1: damage, 2: turns poison is active, 
    let boss_start: Vec<i16> = input.to_vec();

    let mut queue: Vec<(Vec<i16>, Vec<i16>, i16, bool)> = Vec::new();
    queue.push((player_start, boss_start, 0, true));

    let mut least_mana_used = 10000;

    while queue.is_empty() == false {
        let queue_entry = queue.pop().unwrap();
        let mut player = queue_entry.0;
        let mut boss = queue_entry.1;
        let mana_used = queue_entry.2;
        let players_turn = queue_entry.3;

        if mana_used >= least_mana_used {
            continue;
        }

        /////////// first handle the effects
        // handle shield effect
        if player[3] > 0 {
            player[2] = 7;
            player[3] -= 1;
        } else {
            player[2] = 0;
        }

        // handle recharge effect
        if player[4] > 0 {
            player[1] += 101;
            player[4] -= 1;
        }

        // handle poison effect
        if boss[2] > 0 {
            boss[0] -= 3;
            boss[2] -= 1;

            if boss[0] <= 0 {
                if mana_used < least_mana_used {
                    least_mana_used = mana_used;
                }

                continue;
            }
        }

        if players_turn {
            player[0] -= 1;

            if player[0] <= 0 {
                continue;
            }

            // cast magic missile
            if player[1] >= 53 {
                let mut player_copy = player.clone();
                let mut boss_copy = boss.clone();

                boss_copy[0] -= 4;

                let new_mana_used = mana_used + 53;
                if boss_copy[0] <= 0 {
                    if new_mana_used < least_mana_used {
                        least_mana_used = new_mana_used;
                    }
                } else {
                    player_copy[1] -= 53;
                    queue.push((player_copy, boss_copy, new_mana_used, false));
                }
            }
            
            // cast drain
            if player[1] >= 73 {
                let mut player_copy = player.clone();
                let mut boss_copy = boss.clone();

                player_copy[0] += 2;
                boss_copy[0] -= 2;
                
                let new_mana_used = mana_used + 73;
                if boss_copy[0] <= 0 {
                    if new_mana_used < least_mana_used {
                        least_mana_used = new_mana_used;
                    }
                } else {
                    player_copy[1] -= 73;
                    queue.push((player_copy, boss_copy, new_mana_used, false));
                }
            }
            
            // cast shield
            if player[1] >= 113 && player[3] == 0 {
                let mut player_copy = player.clone();

                player_copy[1] -= 113;
                player_copy[3] = 6;
                
                queue.push((player_copy, boss.clone(), mana_used + 113, false));
            }
            
            // cast recharge
            if player[1] >= 229 && player[4] == 0 {
                let mut player_copy = player.clone();

                player_copy[1] -= 229;
                player_copy[4] = 5;

                queue.push((player_copy, boss.clone(), mana_used + 229, false));
            }

            // cast poison
            if player[1] >= 173 && boss[2] == 0{
                let mut player_copy = player.clone();
                let mut boss_copy = boss.clone();

                player_copy[1] -= 173;
                boss_copy[2] = 6;

                queue.push((player_copy, boss_copy, mana_used + 173, false));
            }

        } else {
            player[0] -= cmp::max(1, boss[1] - player[2]);
            if player[0] <= 0 {
                continue;
            }

            queue.push((player.clone(), boss.clone(), mana_used, true));
        }
    }

    least_mana_used
}