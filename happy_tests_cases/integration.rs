//YOU HAVE TO RUN IT ON TEST-NET

use cosmwasm_std::{
    coins, Addr, BlockInfo, Coin, ContractInfo, Env, MessageInfo, Response, Timestamp,
    TransactionInfo,
};
use cosmwasm_storage::to_length_prefixed;
use cosmwasm_vm::testing::{instantiate, mock_info, mock_instance};
use cosmwasm_vm::{from_slice, Storage};

use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cw_escrow::msg::InstantiateMsg;
use cw_escrow::state::State;

// This line will test the output of cargo wasm
static WASM: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/cw_escrow.wasm");
// You can uncomment this line instead to test productionified build from rust-optimizer
// static WASM: &[u8] = include_bytes!("../contract.wasm");

fn init_msg_expire_by_height(height: u64) -> InstantiateMsg {
    InstantiateMsg {
        arbiter: String::from("verifies"),
        recipient: String::from("benefits"),
        end_height: Some(height),
        end_time: None,
    }
}

fn mock_env_info_height(signer: &str, sent: &[Coin], height: u64, time: u64) -> (Env, MessageInfo) {
    let env = Env {
        block: BlockInfo {
            height,
            time: Timestamp::from_nanos(time),
            chain_id: String::from("test"),
        },
        contract: ContractInfo {
            address: Addr::unchecked(MOCK_CONTRACT_ADDR),
        },
        transaction: Some(TransactionInfo { index: 3 }),
    };
    let info = mock_info(signer, sent);
    return (env, info);
}

#[test]
fn proper_initialization() {
    let mut deps = mock_instance(WASM, &[]);

    let msg = init_msg_expire_by_height(2000);
    let (env, info) = mock_env_info_height("creator", &coins(2000, "Jupiter"), 876, 0);
    let res: Response = instantiate(&mut deps, env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    deps.with_storage(|store| {
        let config_key_raw = to_length_prefixed(b"config");
        let state: State =
            from_slice(&store.get(&config_key_raw).0.unwrap().unwrap(), 2048).unwrap();
        assert_eq!(
            state,
            State {
                arbiter: Addr::unchecked("verifies"),
                recipient: Addr::unchecked("benefits"),
                source: Addr::unchecked("creator"),
                end_height: Some(2000),
                end_time: None,
            }
        );
        Ok(())
    })
    .unwrap();
}
