#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::collections::HashMap;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `bool` value on the storage.
        value: i32,
        my_value: HashMap<AccountId, i32>
    }

    impl Incrementer {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value, my_value: HashMap::new() }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self { value: 0, my_value: Default::default()}
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn increment(&mut self, v: i32)  {
            self.value += v
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value_or_zero(&self.env().caller())
        }

        #[ink(message)]
        pub fn incre_mine(&mut self, amount: i32) {
            let caller = self.env().caller();
            let my_value = self.my_value_or_zero(&caller);
            self.my_value.insert(caller, my_value + amount);
        }

        pub fn my_value_or_zero(&self, of: &AccountId) -> i32  {
            *self.my_value.get(of).unwrap_or(&0)
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        #[ink::test]
        fn test_increment() {
            let mut incrementer = Incrementer::new(5);
            assert_eq!(incrementer.get(), 5);

            incrementer.increment(11);
            assert_eq!(incrementer.get(), 16);

            incrementer.increment(-2);
            assert_eq!(incrementer.get(), 14);
        }

        #[ink::test]
        fn test_get_mine() {
           let new_inc = Incrementer::new(15);
           assert_eq!(new_inc.get(), 15); 
           assert_eq!(new_inc.get_mine(), 0);
        }

        #[ink::test]
        fn my_value_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
            contract.incre_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.incre_mine(10);
            assert_eq!(contract.get_mine(), 15);
        }
    }
}
