#[derive(Debug)]
enum ATMState {
    Idle,
    CardInserted { card_number: String },
    PinEntered { card_number: String, balance: i32 },
}

#[derive(Debug)]
enum Action {
    InsertCard(String),
    EnterPin(String),
    Withdraw(i32),
    Eject,
}

fn next_state(state: ATMState, action: Action) -> ATMState {
    match (state, action) {
        (ATMState::Idle, Action::InsertCard(card_number)) => ATMState::CardInserted { card_number },
        (ATMState::CardInserted { card_number }, Action::EnterPin(pin)) => {
            if pin == "1234" {
                ATMState::PinEntered {
                    card_number,
                    balance: 1000,
                }
            } else {
                ATMState::CardInserted { card_number }
            }
        }
        (
            ATMState::PinEntered {
                card_number,
                balance,
            },
            Action::Withdraw(amount),
        ) => {
            if amount <= balance {
                ATMState::PinEntered {
                    card_number,
                    balance: balance - amount,
                }
            } else {
                ATMState::PinEntered {
                    card_number,
                    balance,
                }
            }
        }
        (_, Action::Eject) => ATMState::Idle,
        (s, _) => s,
    }
}

fn main() {
    let state = ATMState::Idle;
    let state = next_state(state, Action::InsertCard("123456".to_string()));
    println!("state: {:?}", state);
    let state = next_state(state, Action::EnterPin("1234".to_string()));
    println!("state: {:?}", state);
    let state = next_state(state, Action::Withdraw(200));
    println!("state: {:?}", state);
    let state = next_state(state, Action::Eject);
    println!("state: {:?}", state);
}

