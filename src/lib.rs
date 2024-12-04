#![no_std]
use gstd::{msg, prelude::*, ActorId, exec};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
extern "C" fn init() {
    let init_config: PebblesInit = msg::load().expect("Failed to decode PebblesInit");
    
    // 验证输入数据
    assert!(init_config.pebbles_count > 0, "Pebbles count must be positive");
    assert!(init_config.max_pebbles_per_turn > 0 && init_config.max_pebbles_per_turn <= init_config.pebbles_count,
            "Invalid max pebbles per turn");

    // 随机选择第一个玩家
    let first_player = if get_random_u32() % 2 == 0 {
        Player::Program
    } else {
        Player::User
    };

    let state = GameState {
        pebbles_count: init_config.pebbles_count,
        max_pebbles_per_turn: init_config.max_pebbles_per_turn,
        pebbles_remaining: init_config.pebbles_count,
        difficulty: init_config.difficulty,
        first_player: first_player.clone(),
        winner: None,
    };

    unsafe {
        GAME_STATE = Some(state);
    }

    // 如果程序先手，进行第一步
    if matches!(first_player, Player::Program) {
        make_program_turn();
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Failed to decode PebblesAction");
    let state = unsafe { GAME_STATE.as_mut().expect("Game state not initialized") };

    match action {
        PebblesAction::Turn(pebbles) => {
            // 验证玩家移动
            assert!(pebbles > 0 && pebbles <= state.max_pebbles_per_turn, 
                   "Invalid number of pebbles");
            assert!(pebbles <= state.pebbles_remaining, 
                   "Not enough pebbles remaining");

            // 更新状态
            state.pebbles_remaining -= pebbles;

            // 检查玩家是否获胜
            if state.pebbles_remaining == 0 {
                state.winner = Some(Player::User);
                msg::reply(PebblesEvent::Won(Player::User), 0)
                    .expect("Failed to send win event");
                return;
            }

            // 程序回合
            make_program_turn();
        }
        PebblesAction::GiveUp => {
            state.winner = Some(Player::Program);
            msg::reply(PebblesEvent::Won(Player::Program), 0)
                .expect("Failed to send give up event");
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            // 重置游戏状态
            *state = GameState {
                pebbles_count,
                max_pebbles_per_turn,
                pebbles_remaining: pebbles_count,
                difficulty,
                first_player: if get_random_u32() % 2 == 0 { Player::User } else { Player::Program },
                winner: None,
            };

            if matches!(state.first_player, Player::Program) {
                make_program_turn();
            }
        }
    }
}

fn make_program_turn() {
    let state = unsafe { GAME_STATE.as_mut().expect("Game state not initialized") };
    
    let pebbles_to_remove = match state.difficulty {
        DifficultyLevel::Easy => {
            // 随机选择移除的石子数量
            (get_random_u32() % state.max_pebbles_per_turn.min(state.pebbles_remaining)) + 1
        }
        DifficultyLevel::Hard => {
            // 实现获胜策略
            calculate_winning_move(state.pebbles_remaining, state.max_pebbles_per_turn)
        }
    };

    state.pebbles_remaining -= pebbles_to_remove;

    if state.pebbles_remaining == 0 {
        state.winner = Some(Player::Program);
        msg::reply(PebblesEvent::Won(Player::Program), 0)
            .expect("Failed to send program win event");
    } else {
        msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0)
            .expect("Failed to send counter turn event");
    }
}

fn calculate_winning_move(remaining: u32, max_per_turn: u32) -> u32 {
    // 在困难模式下计算最佳移动
    // 这里实现一个简单的策略：尽量让剩余数量为 (max_per_turn + 1) 的倍数
    let target = remaining % (max_per_turn + 1);
    if target == 0 {
        1
    } else {
        target
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = unsafe { GAME_STATE.as_ref().expect("Game state not initialized") };
    msg::reply(state.clone(), 0).expect("Failed to send state");
} 