#[cfg(test)]
mod tests {
    use super::*;
    use gtest::{Log, Program, System};

    #[test]
    fn test_init() {
        let sys = System::new();
        let program = Program::current(&sys);

        let init_config = PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            difficulty: DifficultyLevel::Easy,
        };

        let res = program.send(2, init_config);
        assert!(!res.main_failed());
    }

    #[test]
    fn test_user_turn() {
        let sys = System::new();
        let program = Program::current(&sys);

        let init_config = PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            difficulty: DifficultyLevel::Easy,
        };
        program.send(2, init_config);

        let res = program.send(2, PebblesAction::Turn(2));
        assert!(!res.main_failed());
    }

    #[test]
    fn test_game_win() {
        let sys = System::new();
        let program = Program::current(&sys);

        let init_config = PebblesInit {
            pebbles_count: 3,
            max_pebbles_per_turn: 3,
            difficulty: DifficultyLevel::Easy,
        };
        program.send(2, init_config);

        let res = program.send(2, PebblesAction::Turn(3));
        let log = Log::builder()
            .source(program.id())
            .dest(2)
            .payload_bytes(PebblesEvent::Won(Player::User));
        assert!(res.contains(&log));
    }

    #[test]
    fn test_give_up() {
        let sys = System::new();
        let program = Program::current(&sys);

        let init_config = PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            difficulty: DifficultyLevel::Easy,
        };
        program.send(2, init_config);

        let res = program.send(2, PebblesAction::GiveUp);
        let log = Log::builder()
            .source(program.id())
            .dest(2)
            .payload_bytes(PebblesEvent::Won(Player::Program));
        assert!(res.contains(&log));
    }

    #[test]
    fn test_restart() {
        let sys = System::new();
        let program = Program::current(&sys);

        let init_config = PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            difficulty: DifficultyLevel::Easy,
        };
        program.send(2, init_config);

        let res = program.send(
            2,
            PebblesAction::Restart {
                pebbles_count: 15,
                max_pebbles_per_turn: 4,
                difficulty: DifficultyLevel::Hard,
            },
        );
        assert!(!res.main_failed());
    }
