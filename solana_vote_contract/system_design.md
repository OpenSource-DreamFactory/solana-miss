## Implementation approach

我们将使用Rust语言基于Solana区块链开发智能合约，利用Anchor框架简化开发流程。Anchor是一个Rust框架，用于构建Solana的智能合约，提供了易于使用的API和安全的抽象层。我们将通过Anchor定义合约逻辑、数据结构和接口，确保合约的高效和安全。

## File list
```
- Cargo.toml
- lib.rs
- instructions.rs
- state.rs
- errors.rs

## Data structures and interfaces


classDiagram
    class Contract {
        -Whitelist whitelist
        -VoteSettings voteSettings
        +vote(miss_id: String, roma_amount: u64)
        +set_vote_settings(new_settings: VoteSettings)
        +toggle_contract(is_active: bool)
        +get_votes(miss_id: String): u64
    }
    class Whitelist {
        +check(address: String): bool
    }
    class VoteSettings {
        -roma_per_vote: u64
        -contract_active: bool
        +update_settings(new_settings: VoteSettings)
    }
    class Miss {
        -id: String
        -wallet: String
        -votes: u64
        +add_votes(vote_count: u64)
    }
    Contract --> Whitelist
    Contract --> VoteSettings
    Contract --> Miss


## Program call flow


sequenceDiagram
    participant U as User
    participant C as Contract
    participant W as Whitelist
    participant VS as VoteSettings
    participant M as Miss
    U->>C: vote(miss_id, roma_amount)
    C->>W: check(user_address)
    W-->>C: return is_whitelisted
    C->>VS: check_settings()
    VS-->>C: return settings_active
    C->>M: add_votes(calculated_votes)
    M-->>C: update_vote_count
    C-->>U: emit_vote_event(miss_id, miss_wallet, user_address, roma_amount, vote_count)


```

