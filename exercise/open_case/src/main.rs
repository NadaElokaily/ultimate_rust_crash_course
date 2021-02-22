use std::io::stdin;

enum State{
    Locked,
    Unlocked,
    Failed
}

fn main() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    
    loop{
        match state{
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    },
                    Err(_) => {
                        continue;
                    }
                }
                if entry==code{
                    state =State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry){
                    state = State::Failed;
                    continue;
                }
            },
            State::Unlocked => {
                println!("Matched");
                return;
            },
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;
            }
        }
    }

}
