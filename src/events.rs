use ink::env::AccountId;

#[ink::event]
pub struct Transfer {
    #[ink(topic)]
    pub from: AccountId,
    #[ink(topic)]
    pub to: AccountId,
    pub value: u64,
}

#[ink::event]
pub struct Mint {
    #[ink(topic)]
    pub to: AccountId,
    pub value: u64,
}

#[ink::event]
pub struct Burn {
    #[ink(topic)]
    pub from: AccountId,
    pub value: u64,
}
