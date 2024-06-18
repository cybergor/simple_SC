#[cfg(test)]
mod tests {
    use super::*;
    use ink::env::test;

    #[ink::test]
    fn test_new() {
        let contract = contract::MyToken::new(100, 1000);
        let caller = test::default_accounts::<ink::env::DefaultEnvironment>().alice;
        assert_eq!(contract.balance_of(caller), 100);
        assert_eq!(contract.total_supply(), 100);
        assert_eq!(contract.max_supply(), 1000);
    }

    #[ink::test]
    fn test_mint() {
        let mut contract = contract::MyToken::new(100, 1000);
        let bob = test::default_accounts::<ink::env::DefaultEnvironment>().bob;

        assert!(contract.mint(bob, 50).is_ok());
        assert_eq!(contract.balance_of(bob), 50);
        assert_eq!(contract.total_supply(), 150);

        // Minting beyond max supply should fail
        assert!(contract.mint(bob, 900).is_err());
    }

    #[ink::test]
    fn test_transfer() {
        let mut contract = contract::MyToken::new(100, 1000);
        let alice = test::default_accounts::<ink::env::DefaultEnvironment>().alice;
        let bob = test::default_accounts::<ink::env::DefaultEnvironment>().bob;

        assert!(contract.transfer(bob, 30).is_ok());
        assert_eq!(contract.balance_of(alice), 70);
        assert_eq!(contract.balance_of(bob), 30);

        // Transfer with insufficient balance should fail
        assert!(contract.transfer(bob, 100).is_err());
    }

    #[ink::test]
    fn test_burn() {
        let mut contract = contract::MyToken::new(100, 1000);
        let alice = test::default_accounts::<ink::env::DefaultEnvironment>().alice;

        assert!(contract.burn(30).is_ok());
        assert_eq!(contract.balance_of(alice), 70);
        assert_eq!(contract.total_supply(), 70);

        // Burning more than balance should fail
        assert!(contract.burn(100).is_err());
    }

    #[ink::test]
    fn test_only_owner() {
        let mut contract = contract::MyToken::new(100, 1000);
        let bob = test::default_accounts::<ink::env::DefaultEnvironment>().bob;

        // Non-owner should not be able to mint
        assert!(contract.mint(bob, 50).is_err());
    }
}
