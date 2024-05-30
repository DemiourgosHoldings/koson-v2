multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait RewardRateModule {
    fn increment_reward_rate(&self, amount: BigUint) {
        self.current_reward_rate()
            .update(|old_reward_rate| *old_reward_rate += &amount);
    }

    fn update_user_reward_rate(&self, user_address: &ManagedAddress) -> BigUint {
        let current_reward_rate = self.current_reward_rate().get();
        let last_claimed_reward_rate = self.last_claimed_reward_rate(user_address).get();

        let unclaimed_reward_rate = &current_reward_rate - &last_claimed_reward_rate;

        self.last_claimed_reward_rate(user_address)
            .set(&current_reward_rate);

        unclaimed_reward_rate
    }

    #[view(getUnclaimedRewardRate)]
    fn get_unclaimed_reward_rate(&self, user_address: &ManagedAddress) -> BigUint {
        let current_reward_rate = self.current_reward_rate().get();
        let last_claimed_reward_rate = self.last_claimed_reward_rate(user_address).get();

        current_reward_rate - last_claimed_reward_rate
    }

    #[view(getCurrentRewardRate)]
    fn get_current_reward_rate(&self) -> BigUint {
        self.current_reward_rate().get()
    }

    #[view(getLastClaimedRewardRate)]
    fn get_last_claimed_reward_rate(&self, user_address: &ManagedAddress) -> BigUint {
        self.last_claimed_reward_rate(user_address).get()
    }

    #[storage_mapper("current_reward_rate")]
    fn current_reward_rate(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("last_claimed_reward_rate")]
    fn last_claimed_reward_rate(&self, user_address: &ManagedAddress)
        -> SingleValueMapper<BigUint>;
}
