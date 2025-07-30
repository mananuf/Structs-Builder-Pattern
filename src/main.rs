pub mod types;
fn main() {
    run()
}

fn run() {
    let manji = types::user::User::new(
        String::from("Manji"),
        String::from("Gar"),
        String::from("12345678"),
    ).build();

    let james = types::user::User::new(
        String::from("James"),
        String::from("Victor"),
        String::from("12345678"),
    ).age(12)
        .account_type(types::account_type::AccountType::Current)
        .address(types::address::Address {
            country: "Cotonou".to_string(),
            ..Default::default()
        })
        .build();

    print!("{:#?}", manji);
    print!("{:#?}", james)
}