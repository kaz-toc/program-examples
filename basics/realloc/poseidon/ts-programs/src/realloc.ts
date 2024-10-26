import { Account, Pubkey, Result, i64, u8, Signer, u64 } from "@solanaturbine/poseidon";

// creating a class VoteProgram is similar to creating a creating a mod in anchor with all the instructions inside
export default class ReallocProgram {
    static PROGRAM_ID = new Pubkey("HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1");

    initialize(state: MessageState, user: Signer): Result {
        state.derive(["realloc"])
            .init()
        state.value = new i64(0)
    }

    update(state: MessageState, value: i64): Result {
        state.derive(["reallod"])
        state.value = value
    }
}

// define custom accounts by creating an interface which extends class Account
export interface MessageState extends Account {
    value: i64,
}

