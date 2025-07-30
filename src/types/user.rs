use crate::types::account_type::AccountType;
use crate::types::address::Address;

#[derive(Debug)]
pub struct User {
    pub fisrtname: String, // required
    pub lastname: String, // required
    pub account_number: String, // required
    pub account_type: AccountType, // optional
    pub age: u8, // optional
    pub address:Address // optional
}

impl User {
    pub fn new(fisrtname: String, lastname: String, account_number: String) -> UserBuilder {
        UserBuilder {
            fisrtname,
            lastname,
            account_number,
            ..Default::default()
        }
    }
}


#[derive(Default, Debug)]
pub struct UserBuilder {
    pub fisrtname: String, // required
    pub lastname: String, // required
    pub account_number: String, // required
    pub account_type: Option<AccountType>, // optional
    pub age: Option<u8>, // optional
    pub address: Option<Address> // optional
}

impl UserBuilder {
    pub fn account_type(&mut self, account_type: AccountType) -> &mut Self {
        self.account_type = Some(account_type); // Some(Savings) || Some(Current)
        self
    }

    pub fn age(&mut self, age: u8) -> &mut Self {
        self.age = Some(age);
        self
    }

    pub fn address(&mut self, address: Address) -> &mut Self {
        self.address = Some(address);
        self
    }

    pub fn build(&mut self) -> User {
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
