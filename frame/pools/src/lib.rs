//! Delegation pools for nominating in `pallet-staking`.
//!
//! Each pool is represented by: (the actively staked funds), a rewards pool (the
//! rewards earned by the actively staked funds) and a group of unbonding pools (pools )
//!
//! * primary pool: This pool represents the actively staked funds ...
//! * rewards pool: The rewards earned by actively staked funds. Delegator can withdraw rewards once
//! they

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure,
	pallet_prelude::*,
	storage::bounded_btree_map::BoundedBTreeMap,
	traits::{Currency, ExistenceRequirement, Get},
};
use scale_info::TypeInfo;
use sp_arithmetic::{FixedPointNumber, FixedU128};
use sp_runtime::traits::{AtLeast32BitUnsigned, Convert, Saturating, Zero};

pub use pallet::*;
pub(crate) const LOG_TARGET: &'static str = "runtime::pools";

// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] 👜", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

type PoolId = u32;
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type SharesOf<T> = BalanceOf<T>;
type SubPoolsWithEra<T> = BoundedBTreeMap<<T as Config>::EraIndex, UnbondPool<T>, MaxUnbonding<T>>;

pub trait NominationProviderTrait<Balance, AccountId, EraIndex> {
	/// The minimum amount necessary to bond to be a nominator. This does not necessarily mean the
	/// nomination will be counted in an election, but instead just enough to be stored as a
	/// nominator (e.g. in the bags-list of polkadot)
	fn minimum_bond() -> Balance;

	/// The current era for the elections system
	fn current_era() -> EraIndex;

	/// Wether or not the elections system has an ongoing election. If there is an ongoing election
	/// it is assumed that any new pool joiner's funds will not start earning rewards until the
	/// following era.
	fn is_ongoing_election() -> bool;

	/// Balance `controller` has bonded for nominating.
	fn bonded_balance(controller: &AccountId) -> Balance;

	fn bond_extra(controller: &AccountId, extra: Balance) -> DispatchResult;

	fn unbond(controller: &AccountId, value: Balance) -> DispatchResult;

	/// Number of eras that staked funds must remain bonded for.
	fn bond_duration() -> EraIndex;
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[codec(mel_bound(T: Config))]
#[scale_info(skip_type_params(T))]
pub struct Delegator<T: Config> {
	pool: PoolId,
	/// The quantity of shares this delegator has in the p
	shares: SharesOf<T>,
	/// The reward pools total earnings _ever_ the last time this delegator claimed a payout.
	/// Assumiing no massive burning events, we expect this value to always be below total
	/// issuance. This value lines up with the `RewardPool.total_earnings` after a delegator claims
	/// a payout. TODO ^ double check the above is an OK assumption
	reward_pool_total_earnings: BalanceOf<T>,
	/// The era this delegator started unbonding at.
	unbonding_era: Option<T::EraIndex>,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[codec(mel_bound(T: Config))]
#[scale_info(skip_type_params(T))]
pub struct Pool<T: Config> {
	shares: SharesOf<T>, // Probably needs to be some type of BigUInt
	// The _Stash_ and _Controller_ account for the pool.
	account_id: T::AccountId,
}

impl<T: Config> Pool<T> {
	/// Get the amount of shares to issue for some new funds that will be bonded
	/// in the pool.
	///
	/// * `new_funds`: Incoming funds to be bonded against the pool.
	/// * `bonded_balance`: Current bonded balance of the pool.
	fn shares_to_issue(
		&self,
		new_funds: BalanceOf<T>,
		bonded_balance: BalanceOf<T>,
	) -> SharesOf<T> {
		if bonded_balance.is_zero() || self.shares.is_zero() {
			debug_assert!(bonded_balance.is_zero() && self.shares.is_zero());

			// all pools start with a 1:1 ratio of balance:shares
			new_funds
		} else {
			let shares_per_balance = {
				let balance = T::BalanceToU128::convert(bonded_balance);
				let shares = T::BalanceToU128::convert(self.shares);
				FixedU128::saturating_from_rational(shares, balance)
			};
			let new_funds = T::BalanceToU128::convert(new_funds);

			T::U128ToBalance::convert(shares_per_balance.saturating_mul_int(new_funds))
		}
	}

	/// Based on the given shares, unbond the equivalent balance, update the pool accordingly, and
	/// return the balance unbonded.
	fn balance_to_unbond(
		&self,
		delegator_shares: SharesOf<T>,
		bonded_balance: BalanceOf<T>,
	) -> BalanceOf<T> {
		if bonded_balance.is_zero() || delegator_shares.is_zero() {
			// There is nothing to unbond
			return Zero::zero();
		}

		let balance_per_share = {
			let balance = T::BalanceToU128::convert(bonded_balance);
			let shares = T::BalanceToU128::convert(self.shares);
			FixedU128::saturating_from_rational(balance, shares)
		};
		let delegator_shares = T::BalanceToU128::convert(delegator_shares);

		T::U128ToBalance::convert(balance_per_share.saturating_mul_int(delegator_shares))
	}
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[codec(mel_bound(T: Config))]
#[scale_info(skip_type_params(T))]
pub struct RewardPool<T: Config> {
	// TODO look into using the BigUInt
	/// The balance of this reward pool after the last claimed payout.
	balance: BalanceOf<T>,
	/// The shares of this reward pool after the last claimed payout
	shares: BalanceOf<T>, // TODO maybe MaxEncodedLen or something
	/// The total earnings _ever_ of this reward pool after the last claimed payout. I.E. the sum
	/// of all incoming balance.
	total_earnings: BalanceOf<T>,
	/// The reward destination for the pool.
	account_id: T::AccountId,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[codec(mel_bound(T: Config))]
#[scale_info(skip_type_params(T))]
struct UnbondPool<T: Config> {
	shares: SharesOf<T>,
	balance: BalanceOf<T>,
}

impl<T: Config> UnbondPool<T> {
	fn shares_to_issue(&self, new_funds: BalanceOf<T>) -> SharesOf<T> {
		if self.balance.is_zero() || self.shares.is_zero() {
			debug_assert!(self.balance.is_zero() && self.shares.is_zero());

			// all pools start with a 1:1 ratio of balance:shares
			new_funds
		} else {
			let shares_per_balance = {
				let balance = T::BalanceToU128::convert(self.balance);
				let shares = T::BalanceToU128::convert(self.shares);
				FixedU128::saturating_from_rational(shares, balance)
			};
			let new_funds = T::BalanceToU128::convert(new_funds);

			T::U128ToBalance::convert(shares_per_balance.saturating_mul_int(new_funds))
		}
	}

	fn balance_to_unbond(&self, delegator_shares: SharesOf<T>) -> BalanceOf<T> {
		if self.balance.is_zero() || delegator_shares.is_zero() {
			// There is nothing to unbond
			return Zero::zero();
		}

		let balance_per_share = {
			let balance = T::BalanceToU128::convert(self.balance);
			let shares = T::BalanceToU128::convert(self.shares);
			FixedU128::saturating_from_rational(balance, shares)
		};
		let delegator_shares = T::BalanceToU128::convert(delegator_shares);

		T::U128ToBalance::convert(balance_per_share.saturating_mul_int(delegator_shares))
	}
}

impl<T: Config> Default for UnbondPool<T> {
	fn default() -> Self {
		Self { shares: Zero::zero(), balance: Zero::zero() }
	}
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
#[codec(mel_bound(T: Config))]
#[scale_info(skip_type_params(T))]
struct SubPoolContainer<T: Config> {
	/// A general, era agnostic pool of funds that have fully unbonded. The pools
	/// of `self.with_era` will lazily be merged into into this pool if they are
	/// older then `current_era - T::MAX_UNBONDING`.
	no_era: UnbondPool<T>,
	/// Map of era => unbond pools.
	with_era: SubPoolsWithEra<T>,
}

impl<T: Config> SubPoolContainer<T> {
	/// Merge the oldest unbonding pool with an era into the general unbond pool with no associated
	/// era.
	fn maybe_merge_pools(mut self, current_era: T::EraIndex) -> Self {
		if current_era < T::MAX_UNBONDING.into() {
			// For the first `T::MAX_UNBONDING` eras of the chain we don't need to do anything.
			// I.E. if `MAX_UNBONDING` is 5 and we are in era 4 we can add a pool for this era and
			// have exactly `MAX_UNBONDING` pools.
			return self;
		}

		//  I.E. if `MAX_UNBONDING` is 5 and current era is 10, we only want to retain pools 6..=10.
		let oldest_era_to_keep = current_era - (T::MAX_UNBONDING.saturating_add(1)).into();

		let eras_to_remove: Vec<_> =
			self.with_era.keys().cloned().filter(|era| *era < oldest_era_to_keep).collect();
		for era in eras_to_remove {
			if let Some(p) = self.with_era.remove(&era) {
				self.no_era.shares.saturating_add(p.shares);
				self.no_era.balance.saturating_add(p.balance);
			} else {
				// lol
			}
		}

		self
	}
}

// TODO figure out why the Default derive did not work for SubPoolContainer
impl<T: Config> sp_std::default::Default for SubPoolContainer<T> {
	fn default() -> Self {
		Self { no_era: UnbondPool::<T>::default(), with_era: SubPoolsWithEra::<T>::default() }
	}
}

// Wrapper for `T::MAX_UNBONDING` to satisfy `trait Get`.
pub struct MaxUnbonding<T>(PhantomData<T>);
impl<T: Config> Get<u32> for MaxUnbonding<T> {
	fn get() -> u32 {
		T::MAX_UNBONDING
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::{ensure_signed, pallet_prelude::*};

	#[pallet::pallet]
	#[pallet::generate_store(pub(crate) trait Store)]
	#[pallet::generate_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight information for extrinsics in this pallet.
		// type WeightInfo: weights::WeightInfo;

		/// The nominating balance.
		type Currency: Currency<Self::AccountId>;

		// Infallible method for converting `Currency::Balance` to `u128`.
		type BalanceToU128: Convert<BalanceOf<Self>, u128>;

		// Infallible method for converting `u128` to `Currency::Balance`.
		type U128ToBalance: Convert<u128, BalanceOf<Self>>;

		/// The type for unique era indexes. Likely comes from what implements `NominationProvider`.
		type EraIndex: Member
			+ Parameter
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ TypeInfo;

		/// The interface for nominating.
		type NominationProvider: NominationProviderTrait<
			BalanceOf<Self>,
			Self::AccountId,
			Self::EraIndex,
		>;

		/// The maximum amount of eras an unbonding pool can exist prior to being merged with the
		/// "average" (TODO need better terminology) unbonding pool.
		const MAX_UNBONDING: u32;

		// MaxPools
	}

	/// Active delegators.
	#[pallet::storage]
	pub(crate) type Delegators<T: Config> =
		CountedStorageMap<_, Twox64Concat, T::AccountId, Delegator<T>>;

	/// Bonded pools.
	#[pallet::storage]
	pub(crate) type PrimaryPools<T: Config> = CountedStorageMap<_, Twox64Concat, PoolId, Pool<T>>;

	/// Reward pools. This is where there rewards for each pool accumulate. When a delegators payout
	/// is claimed, the balance comes out fo the reward pool.
	#[pallet::storage]
	pub(crate) type RewardPools<T: Config> =
		CountedStorageMap<_, Twox64Concat, PoolId, RewardPool<T>>;

	/// Groups of unbonding pools. Each group of unbonding pools belongs to a primary pool,
	/// hence the name sub-pools.
	#[pallet::storage]
	pub(crate) type SubPools<T: Config> =
		CountedStorageMap<_, Twox64Concat, PoolId, SubPoolContainer<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		// TODO: these operations are per delegator - so these events could become decently noisy
		// if things scale a lot - consider not including these.
		Joined { delegator: T::AccountId, pool: PoolId, bonded: BalanceOf<T> },
		Payout { delegator: T::AccountId, pool: PoolId, payout: BalanceOf<T> },
	}

	#[pallet::error]
	#[cfg_attr(test, derive(PartialEq))]
	pub enum Error<T> {
		/// The given (primary) pool id does not exist.
		PoolNotFound,
		/// The given account is not a delegator.
		DelegatorNotFound,
		// The given reward pool does not exist. In all cases this is a system logic error.
		RewardPoolNotFound,
		/// The account is already delegating in another pool. An account may only belong to one
		/// pool at a time.
		AccountBelongsToOtherPool,
		/// The pool has insufficient balance to bond as a nominator.
		InsufficientBond,
		/// The delegator is already unbonding.
		AlreadyUnbonding,
		/// The delegator is not unbonding and thus cannot withdraw funds.
		NotUnbonding,
		/// Unbonded funds cannot be withdrawn yet because the bond duration has not passed.
		NotUnbondedYet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Join a pre-existing pool. Note that an account can only be a member of a single pool.
		#[pallet::weight(666)]
		pub fn join(origin: OriginFor<T>, amount: BalanceOf<T>, target: PoolId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// if a delegator already exists that means they already belong to a pool
			ensure!(!Delegators::<T>::contains_key(&who), Error::<T>::AccountBelongsToOtherPool);

			// Ensure that the `target` pool exists
			let mut primary_pool =
				PrimaryPools::<T>::get(target).ok_or(Error::<T>::PoolNotFound)?;
			// And that `amount` will meet the minimum bond
			let old_free_balance = T::Currency::free_balance(&primary_pool.account_id);
			ensure!(
				old_free_balance.saturating_add(amount) >= T::NominationProvider::minimum_bond(),
				Error::<T>::InsufficientBond
			);

			// Transfer the funds to be bonded from `who` to the pools account so the pool can then
			// go bond them.
			// Note importantly that we can't error after this transfer goes through.
			// TODO I assume this does proper keep alive checks etc but need to double check
			T::Currency::transfer(
				&who,
				&primary_pool.account_id,
				amount,
				ExistenceRequirement::KeepAlive,
			)?;
			// this should now include the transferred balance
			let new_free_balance = T::Currency::free_balance(&primary_pool.account_id);
			// we get the exact amount we can bond extra
			let exact_amount_to_bond = new_free_balance.saturating_sub(old_free_balance);

			let bonded_balance = T::NominationProvider::bonded_balance(&primary_pool.account_id);
			// issue the new shares
			let new_shares = primary_pool.shares_to_issue(exact_amount_to_bond, bonded_balance);
			primary_pool.shares = primary_pool.shares.saturating_add(new_shares);
			let delegator = Delegator::<T> {
				pool: target,
				shares: new_shares,
				// TODO this likely needs to be the reward pools total earnings at this block
				// - go and double check
				reward_pool_total_earnings: Zero::zero(),
				unbonding_era: None,
			};

			// Do bond extra
			T::NominationProvider::bond_extra(&primary_pool.account_id, exact_amount_to_bond)?;

			// Write the pool and delegator to storage
			Delegators::insert(who.clone(), delegator);
			PrimaryPools::insert(target, primary_pool);

			// And finally emit an event to confirm the exact amount bonded
			Self::deposit_event(Event::<T>::Joined {
				delegator: who,
				pool: target,
				bonded: exact_amount_to_bond,
			});

			Ok(())
		}

		/// Claim a payout for a delegator can use this to claim their payout based on the
		/// rewards /// that the pool has accumulated since their last claimed payout (OR since
		///
		/// joining if this /// is there for). The payout will go to the delegators account.
		///
		/// This extrinsic is permisionless in the sense that any account can call it for any
		/// delegator in the system.
		#[pallet::weight(666)]
		pub fn claim_payout(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let delegator = Delegators::<T>::get(&who).ok_or(Error::<T>::DelegatorNotFound)?;
			let primary_pool = PrimaryPools::<T>::get(&delegator.pool).ok_or_else(|| {
				log!(error, "A primary pool could not be found, this is a system logic error.");
				debug_assert!(
					false,
					"A primary pool could not be found, this is a system logic error."
				);
				Error::<T>::PoolNotFound
			})?;

			Self::do_reward_payout(who, delegator, &primary_pool)?;

			Ok(())
		}

		/// Unbond _all_ funds.
		#[pallet::weight(666)]
		pub fn unbond(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let delegator = Delegators::<T>::get(&who).ok_or(Error::<T>::DelegatorNotFound)?;
			let mut primary_pool =
				PrimaryPools::<T>::get(delegator.pool).ok_or(Error::<T>::PoolNotFound)?;

			// Claim the the payout prior to unbonding. Once the user is unbonding their shares
			// no longer exist in the primary pool and thus they can no longer claim their payouts.
			// It is not strictly necessary to claim the rewards, but we do it here for UX.
			Self::do_reward_payout(who.clone(), delegator, &primary_pool)?;

			// Re-fetch the delegator because they where updated by `do_reward_payout`.
			let mut delegator = Delegators::<T>::get(&who).ok_or(Error::<T>::DelegatorNotFound)?;
			// Note that we lazily create the unbonding pools here if they don't already exist
			let sub_pools = SubPools::<T>::get(delegator.pool).unwrap_or_default();
			// TODO double check if we need to count for elections when
			// the unbonding era.
			let current_era = T::NominationProvider::current_era();

			let bonded_balance = T::NominationProvider::bonded_balance(&primary_pool.account_id);
			let balance_to_unbond =
				primary_pool.balance_to_unbond(delegator.shares, bonded_balance);

			// Update the primary pool. Note that we must do this *after* calculating the balance
			// to unbond.
			primary_pool.shares = primary_pool.shares.saturating_sub(delegator.shares);

			// Unbond in the actual underlying pool - we can't fail after this
			// TODO - we can only do this for as many locking chunks are accepted
			T::NominationProvider::unbond(&primary_pool.account_id, balance_to_unbond)?;

			// Merge any older pools into the general, era agnostic unbond pool. Note that we do
			// this before inserting to ensure we don't go over the max unbonding pools.
			let mut sub_pools = sub_pools.maybe_merge_pools(current_era);

			// Update the unbond pool associated with the current era with the
			// unbonded funds. Note that we lazily create the unbond pool if it
			// does not yet exist.
			// let unbond_pool = sub_pools
			// 	.with_era
			// 	.entry(current_era)
			// 	.or_insert_with(|| UnbondPool::<T>::default());
			{
				let unbond_pool = sub_pools.with_era.get_mut(&current_era).unwrap();
				let shares_to_issue = unbond_pool.shares_to_issue(balance_to_unbond);
				unbond_pool.shares = unbond_pool.shares.saturating_add(shares_to_issue);
				unbond_pool.balance = unbond_pool.balance.saturating_add(balance_to_unbond);
			}

			delegator.unbonding_era = Some(current_era);

			// Now that we know everything has worked write the items to storage.
			PrimaryPools::insert(delegator.pool, primary_pool);
			SubPools::insert(delegator.pool, sub_pools);
			Delegators::insert(who, delegator);

			Ok(())
		}

		#[pallet::weight(666)]
		pub fn withdraw_unbonded(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let delegator = Delegators::<T>::take(&who).ok_or(Error::<T>::DelegatorNotFound)?;

			let unbonding_era = delegator.unbonding_era.ok_or(Error::<T>::NotUnbonding)?;
			let current_era = T::NominationProvider::current_era();
			if current_era.saturating_sub(unbonding_era) < T::NominationProvider::bond_duration() {
				return Err(Error::<T>::NotUnbondedYet.into());
			};

			let mut sub_pools = SubPools::<T>::get(delegator.pool).unwrap_or_default();

			let balance_to_unbond = if let Some(pool) = sub_pools.with_era.get_mut(&current_era) {
				let balance_to_unbond = pool.balance_to_unbond(delegator.shares);
				pool.shares = pool.shares.saturating_sub(delegator.shares);
				pool.balance = pool.balance.saturating_sub(balance_to_unbond);

				balance_to_unbond
			} else {
				// A pool does not belong to this era, so it must have been merged to the era-less pool.
				let balance_to_unbond = sub_pools.no_era.balance_to_unbond(delegator.shares);
				sub_pools.no_era.shares = sub_pools.no_era.shares.saturating_sub(delegator.shares);
				sub_pools.no_era.balance =
					sub_pools.no_era.balance.saturating_sub(balance_to_unbond);

				balance_to_unbond
			};

			let primary_pool =
				PrimaryPools::<T>::get(delegator.pool).ok_or(Error::<T>::PoolNotFound)?;
			T::Currency::transfer(
				&primary_pool.account_id,
				&who,
				balance_to_unbond,
				ExistenceRequirement::AllowDeath,
			)?;

			SubPools::<T>::insert(delegator.pool, sub_pools);

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Calculate the rewards for `delegator`.
	fn calculate_delegator_payout(
		primary_pool: &Pool<T>,
		mut reward_pool: RewardPool<T>,
		mut delegator: Delegator<T>,
	) -> Result<(RewardPool<T>, Delegator<T>, BalanceOf<T>), DispatchError> {
		// If the delegator is unbonding they cannot claim rewards. Note that when the delagator
		// goes to unbond, the unbond function should claim rewards for the final time.
		ensure!(delegator.unbonding_era.is_none(), Error::<T>::AlreadyUnbonding);

		let current_balance = T::Currency::free_balance(&reward_pool.account_id);

		// The earnings since the last claimed payout
		let new_earnings = current_balance.saturating_sub(reward_pool.balance);

		// The lifetime earnings of the of the reward pool
		let new_total_earnings = new_earnings.saturating_add(reward_pool.total_earnings);

		// The new shares that will be added to the pool. For every unit of balance that has
		// been earned by the reward pool, we inflate the reward pool shares by
		// `primary_pool.total_shares`. In effect this allows each, single unit of balance (e.g.
		// plank) to be divvied up pro-rata among delegators based on shares.
		// TODO this needs to be some sort of BigUInt arithmetic
		let new_shares = primary_pool.shares.saturating_mul(new_earnings);

		// The shares of the reward pool after taking into account the new earnings
		let current_shares = reward_pool.shares.saturating_add(new_shares);

		// The rewards pool's earnings since the last time this delegator claimed a payout
		let new_earnings_since_last_claim =
			new_total_earnings.saturating_sub(delegator.reward_pool_total_earnings);
		// The shares of the reward pool that belong to the delegator.
		let delegator_virtual_shares =
			delegator.shares.saturating_mul(new_earnings_since_last_claim);

		let delegator_payout = {
			let delegator_ratio_of_shares = FixedU128::saturating_from_rational(
				T::BalanceToU128::convert(delegator_virtual_shares),
				T::BalanceToU128::convert(current_shares),
			);

			let payout = delegator_ratio_of_shares
				.saturating_mul_int(T::BalanceToU128::convert(current_balance));
			T::U128ToBalance::convert(payout)
		};

		// Record updates
		delegator.reward_pool_total_earnings = new_total_earnings;
		reward_pool.shares = current_shares.saturating_sub(delegator_virtual_shares);
		reward_pool.balance = current_balance;
		reward_pool.total_earnings = new_total_earnings;

		Ok((reward_pool, delegator, delegator_payout))
	}

	/// Transfer the delegator their payout from the pool and deposit the corresponding event.
	fn transfer_reward(
		reward_pool: &T::AccountId,
		delegator: T::AccountId,
		pool: PoolId,
		payout: BalanceOf<T>,
	) -> Result<(), DispatchError> {
		T::Currency::transfer(
			reward_pool,
			&delegator,
			payout,
			// TODO double check we are ok with dusting the account - If their is a very high
			// ED this could lead to a non-negligible loss of rewards
			ExistenceRequirement::AllowDeath, // Dust may be lost here
		)?;
		Self::deposit_event(Event::<T>::Payout { delegator, pool, payout });

		Ok(())
	}

	fn do_reward_payout(
		delegator_id: T::AccountId,
		delegator: Delegator<T>,
		primary_pool: &Pool<T>,
	) -> DispatchResult {
		let reward_pool = RewardPools::<T>::get(&delegator.pool).ok_or_else(|| {
			log!(error, "A reward pool could not be found, this is a system logic error.");
			debug_assert!(false, "A reward pool could not be found, this is a system logic error.");
			Error::<T>::RewardPoolNotFound
		})?;

		let (reward_pool, delegator, delegator_payout) =
			Self::calculate_delegator_payout(primary_pool, reward_pool, delegator)?;

		// Transfer payout to the delegator.
		Self::transfer_reward(
			&reward_pool.account_id,
			delegator_id.clone(),
			delegator.pool,
			delegator_payout,
		)?;

		// Write the updated delegator and reward pool to storage
		RewardPools::insert(delegator.pool, reward_pool);
		Delegators::insert(delegator_id, delegator);

		Ok(())
	}
}

// TODO
// - rebond_rewards
// - force pool creation
// - force pool update
// - force pool delete?

// impl<T: Config> Pallet<T> {
// 	do_create_pool(
// 		creator: T::AccountId,
// 		targets: Vec<T::AccountId>,
// 		amount: BalanceOf<T>
// 	) -> DispatchResult {
// Create Stash/Controller account based on parent block hash, block number, and extrinsic index
// Create Reward Pool account based on Stash/Controller account
// Move `amount` to the stash / controller account
// Read in `bondable` - the free balance that we can bond after any neccesary reserv etc

// Bond with `amount`, ensuring that it is over the minimum bond (by min)
// (might has need to ensure number of targets etc is valid)

// Generate a pool id (look at how assets IDs are generated for inspiration)

//
// 	}
// }
