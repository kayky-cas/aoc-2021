use std::time::Instant;

#[derive(Clone)]
struct Player {
    pos: usize,
    points: usize,
}

fn day21_1(p1_pos: usize, p2_pos: usize) -> usize {
    let player_1 = Player {
        pos: p1_pos,
        points: 0,
    };

    let player_2 = Player {
        pos: p2_pos,
        points: 0,
    };

    let mut dices = 0;
    let mut current_dice = 0;

    let mut current_player = 0;
    let mut players = [player_1, player_2];

    loop {
        let player = &mut players[current_player % 2];
        current_player += 1;

        let mut skip = 0;

        for _ in 0..3 {
            current_dice += 1;

            if current_dice > 100 {
                current_dice %= 100;
            }

            skip += current_dice;
            dices += 1;
        }

        player.pos += skip;

        if player.pos % 10 == 0 {
            player.pos = 10;
        } else {
            player.pos %= 10;
        }

        player.points += player.pos;

        if player.points >= 1000 {
            break;
        }
    }

    let loser = players[current_player % 2].points;

    return loser * dices;
}

fn day21_2aux(
    mut current_player: usize,
    players: [Player; 2],
    universes: usize,
    player_wins: &mut [usize; 2],
    possibilities: &[(usize, usize)],
) {
    if players[0].points >= 21 {
        player_wins[0] += universes;
        return;
    }

    if players[1].points >= 21 {
        player_wins[1] += universes;
        return;
    }

    let next_player = (current_player + 1) % 2;
    current_player %= 2;

    for (skip, times) in possibilities {
        let mut players = players.clone();

        players[current_player].pos += skip;

        if players[current_player].pos > 10 {
            players[current_player].pos -= 10;
        }

        players[current_player].points += players[current_player].pos;

        day21_2aux(
            next_player,
            players,
            universes * times,
            player_wins,
            possibilities,
        )
    }
}

fn day21_2(p1_pos: usize, p2_pos: usize) -> usize {
    let possibilities = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut player_wins = [0, 0];

    let player_1 = Player {
        pos: p1_pos,
        points: 0,
    };

    let player_2 = Player {
        pos: p2_pos,
        points: 0,
    };

    day21_2aux(0, [player_1, player_2], 1, &mut player_wins, &possibilities);

    return player_wins[0].max(player_wins[1]);
}

fn main() {
    let instant = Instant::now();
    println!("Part 1 = {} in {:?}", day21_1(2, 5), instant.elapsed());
    let instant = Instant::now();
    println!("Part 2 = {} in {:?}", day21_2(2, 5), instant.elapsed());
}
