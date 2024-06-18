use ink::env::AccountId;

pub struct AccessControl;

impl AccessControl {
    /// Проверяет, является ли вызывающий аккаунт владельцем контракта.
    pub fn only_owner(caller: AccountId, owner: AccountId) -> Result<(), &'static str> {
        if caller != owner {
            return Err("Caller is not the owner");
        }
        Ok(())
    }
}
