use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Hash, PartialEq, Eq)]
enum Team {
    Sillazo,
    Francia,
    Exiled,
    SierraMágina,
    NewJersey,
    Gamonal,
    Catalonia,
    Narraweena,
    Chocolate,
    Losantos,
    Withington,
    Blazers,
    Indiana,
    MiRondo,
    DonJoséLuis,
    Historiador,
    Valdebernardo,
    ElTruua,
}

#[derive(Debug)]
struct Record {
    wins: u8,
    avg: Decimal,
    games_played: u8,
}

impl Record {
    fn new(wins: u8, avg: f64, total_games: u8) -> Self {
        Self {
            wins,
            avg: Decimal::from_f64(avg).expect("avg to fit into Decimal"),
            games_played: total_games,
        }
    }

    fn add_win(&mut self, game_points: Decimal) {
        self.update_avg(game_points);
        self.wins += 1;
    }

    fn add_loss(&mut self, game_points: Decimal) {
        self.update_avg(game_points);
    }

    fn update_avg(&mut self, game_points: Decimal) {
        let games_played: Decimal = self
            .games_played
            .try_into()
            .expect("games played to fit into Decimal");

        let season_points = self.avg * games_played;

        self.avg = (season_points + game_points) / (games_played + Decimal::ONE);
        self.games_played += 1;
    }
}

#[derive(Debug)]
struct Standings(HashMap<Team, Record>);

impl Standings {
    fn add_game(
        &mut self,
        (away, away_score): (Team, Decimal),
        (home, home_score): (Team, Decimal),
    ) {
        if home_score == away_score {
            panic!("basketball games cannot be drawn!");
        }

        let home_wins = home_score > away_score;

        self.0.entry(home).and_modify(|record| {
            if home_wins {
                record.add_win(home_score);
            } else {
                record.add_loss(home_score);
            }
        });

        self.0.entry(away).and_modify(|record| {
            if home_wins {
                record.add_loss(home_score);
            } else {
                record.add_win(home_score);
            }
        });
    }
}

impl Display for Standings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut standings = self.0.iter().collect::<Vec<_>>();
        standings.sort_by(|a, b| b.1.wins.cmp(&a.1.wins).then_with(|| b.1.avg.cmp(&a.1.avg)));

        writeln!(f, "| Team | Record | Average |")?;
        writeln!(f, "| --- | --- | --- |")?;

        for (
            team,
            Record {
                wins,
                avg,
                games_played,
            },
        ) in standings.iter()
        {
            let losses = games_played - wins;
            let avg =
                avg.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero);

            writeln!(f, "| {:} | {:}-{:} | {:} |", team, wins, losses, avg)?
        }

        Ok(())
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Team::*;
        let name = match self {
            Sillazo => "Sillazo Krstićs",
            Francia => "Francia Franceses",
            Exiled => "Exiled Brownies",
            SierraMágina => "Sierra Mágina PERs",
            NewJersey => "New Jersey Kinkis",
            Gamonal => "Gamonal Sinoneveros",
            Catalonia => "Catalonia Midecráneos",
            Narraweena => "Narraweena Whippets",
            Chocolate => "Chocolate Milk",
            Losantos => "Losantos Lakers",
            Withington => "Withington Aparachiquis",
            Blazers => "Blazers Argento",
            Indiana => "Indiana Clínicas",
            MiRondo => "MiRondo Randoms",
            DonJoséLuis => "Don José Luis 4 ermitas",
            Historiador => "Historiador C.Sirera",
            Valdebernardo => "Valdebernardo Caprichitos",
            ElTruua => "EL TRUUA BATTLELISTA",
        };

        write!(f, "{}", name)
    }
}

fn main() {
    use Team::*;

    let standings_g57 = Standings(HashMap::from_iter([
        (Sillazo, Record::new(39, 197.8, 57)),
        (Francia, Record::new(38, 207.1, 57)),
        (Exiled, Record::new(35, 201.5, 57)),
        (SierraMágina, Record::new(34, 200.8, 57)),
        (NewJersey, Record::new(34, 193.5, 57)),
        (Gamonal, Record::new(33, 183.1, 57)),
        (Catalonia, Record::new(31, 191.9, 57)),
        (Narraweena, Record::new(31, 189.6, 57)),
        (Chocolate, Record::new(30, 196.7, 57)),
        (Losantos, Record::new(30, 194.1, 57)),
        (Withington, Record::new(29, 184.7, 57)),
        (Blazers, Record::new(28, 172.3, 57)),
        (Indiana, Record::new(27, 184.8, 57)),
        (MiRondo, Record::new(24, 180.0, 57)),
        (DonJoséLuis, Record::new(23, 175.3, 57)),
        (Historiador, Record::new(21, 169.7, 57)),
        (Valdebernardo, Record::new(20, 172.0, 57)),
        (ElTruua, Record::new(6, 145.2, 57)),
    ]));

    let standings_g58 = {
        let mut standings = standings_g57;
        standings.add_game((Historiador, dec!(171.3)), (Exiled, dec!(229.7)));
        standings.add_game((Withington, dec!(196.4)), (Chocolate, dec!(215.8)));
        standings.add_game((Francia, dec!(206.7)), (Narraweena, dec!(233.3)));
        standings.add_game((ElTruua, dec!(149.4)), (SierraMágina, dec!(210.8)));
        standings.add_game((Gamonal, dec!(161.35)), (Sillazo, dec!(204.2)));
        standings.add_game((DonJoséLuis, dec!(165.45)), (Valdebernardo, dec!(190.55)));
        standings.add_game((Blazers, dec!(182.15)), (MiRondo, dec!(171.2)));
        standings.add_game((Catalonia, dec!(214.1)), (Losantos, dec!(192.3)));
        standings.add_game((Indiana, dec!(111.3)), (NewJersey, dec!(174)));

        standings
    };

    let standings_g59 = {
        let mut standings = standings_g58;
        standings.add_game((Exiled, dec!(218.8)), (NewJersey, dec!(196.75)));
        standings.add_game((SierraMágina, dec!(232.2)), (Withington, dec!(193)));
        standings.add_game((Narraweena, dec!(212.95)), (Chocolate, dec!(205.55)));
        standings.add_game((Francia, dec!(221.85)), (Historiador, dec!(199.15)));
        standings.add_game((Sillazo, dec!(200.4)), (ElTruua, dec!(170.9)));
        standings.add_game((Valdebernardo, dec!(169.8)), (Gamonal, dec!(135.65)));
        standings.add_game((MiRondo, dec!(187.35)), (DonJoséLuis, dec!(200.55)));
        standings.add_game((Losantos, dec!(213.35)), (Blazers, dec!(197.8)));
        standings.add_game((Indiana, dec!(195.4)), (Catalonia, dec!(230.35)));

        standings
    };

    let standings_g60 = {
        let mut standings = standings_g59;
        standings.add_game((Francia, dec!(193.15)), (Exiled, dec!(215.4)));
        standings.add_game((Withington, dec!(200.6)), (Sillazo, dec!(218.65)));
        standings.add_game((SierraMágina, dec!(223.6)), (Narraweena, dec!(199.15)));
        standings.add_game((Chocolate, dec!(208.6)), (Historiador, dec!(190.95)));
        standings.add_game((ElTruua, dec!(195.75)), (Valdebernardo, dec!(181.6)));
        standings.add_game((Gamonal, dec!(202)), (MiRondo, dec!(200.9)));
        standings.add_game((DonJoséLuis, dec!(199.25)), (Losantos, dec!(215.5)));
        standings.add_game((Blazers, dec!(193.65)), (Indiana, dec!(175.95)));
        standings.add_game((Catalonia, dec!(197.1)), (NewJersey, dec!(192.35)));

        standings
    };

    println!("{}", standings_g60);
}
