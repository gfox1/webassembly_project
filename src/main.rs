
struct BankAccount {
    balance: i32,
    verified: bool,
}

fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}

fn is_verified (account: &BankAccount) -> Result<bool, bool> {
    return match account.verified {
        true => Ok(true),
        false => Err(false),
    }
}

fn main() {
    let mut total = add(10, 15);
    let mut free_shiping = false;


    match free_shiping {
        true => total = total + 0,
        false => total = total + 5,
    }

    match total {
        50 => println!("does not qualify for free shipping"),
        _ => println!("qualifies for shipping")
    }


    let my_account = BankAccount {
        balance: 20,
        verified: true,
    };

    let verification_status = is_verified(&my_account)
        .expect("Not verified");
    println!("{:?}", verification_status);


}
