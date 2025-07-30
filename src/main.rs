#[derive(Debug, Clone)]
enum AccountType {
    Savings,
    Current
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Savings
    }
}

#[derive(Debug, Clone)]
struct Address {
    country: String,
    state: String,
    city: String
}

impl Default for Address{
    fn default() -> Self {
        Self {
            country: "".to_string(),
            state: "".to_string(),
            city: "".to_string(),
        }
    }
}


#[derive(Debug)]
struct User {
    fisrtname: String, // required
    lastname: String, // required
    account_number: String, // required
    account_type: AccountType, // optional
    age: u8, // optional
    address:Address // optional
}

impl User {
    fn new(fisrtname: String, lastname: String, account_number: String) -> UserBuilder {
        UserBuilder {
            fisrtname,
            lastname,
            account_number,
            ..Default::default()
        }
    }
}


#[derive(Default, Debug)]
struct UserBuilder {
    fisrtname: String, // required
    lastname: String, // required
    account_number: String, // required
    account_type: Option<AccountType>, // optional
    age: Option<u8>, // optional
    address: Option<Address> // optional
}

impl UserBuilder {
    fn account_type(&mut self, account_type: AccountType) -> &mut Self {
        self.account_type = Some(account_type); // Some(Savings) || Some(Current)
        self
    }

    fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    fn address(&mut self, address: Address) -> &mut Self {
        self.address = Some(address);
        self
    }

    fn build(&mut self) -> User {
        User {
            fisrtname: self.fisrtname.clone(),
            lastname: self.lastname.clone(),
            account_number: self.account_number.clone(),
            account_type: self.account_type.clone().unwrap_or_default(), // Some(Current) -> Current
            age: self.age.clone().unwrap_or_default(), // Some(12) -> 12
            address: self.address.clone().unwrap_or_default() // Some(Address) -> Address
        }
    }
}


fn main() {
    let manji = User::new(
        String::from("Manji"),
        String::from("Gar"),
        String::from("12345678"),
    ).build();

    let james = User::new(
        String::from("James"),
        String::from("Victor"),
        String::from("12345678"),
    ).age(12)
    .account_type(AccountType::Current)
    .address(Address {
        country: "Cotonou".to_string(),
        ..Default::default()
    })
    .build();

    print!("{:#?}", manji);
    print!("{:#?}", james)

}