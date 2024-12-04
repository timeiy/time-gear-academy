use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn basic_initialization() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // 测试初始化
    let init_config = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
    };
    let res = program.send(2, init_config);
    assert!(!res.main_failed());
}

#[test]
fn test_game_flow() {
    let sys = System::new();
    let program = Program::current(&sys);

    // 初始化游戏
    let init_config = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
    };
    program.send(2, init_config);

    // 测试玩家回合
    let res = program.send(2, PebblesAction::Turn(2));
    assert!(!res.main_failed());
}
