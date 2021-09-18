//! # Simple disputes
//!
//! Manages market disputes and resolutions.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod simple_disputes_pallet_api;

pub use pallet::*;
pub use simple_disputes_pallet_api::SimpleDisputesPalletApi;

#[frame_support::pallet]
mod pallet {
    use crate::SimpleDisputesPalletApi;
    use core::marker::PhantomData;
    use frame_support::{
        dispatch::DispatchResult,
        traits::{Currency, Get, Hooks, IsType},
        PalletId,
    };
    use sp_runtime::DispatchError;
    use zeitgeist_primitives::{
        traits::{DisputeApi, Swaps, ZeitgeistMultiReservableCurrency},
        types::{Asset, Market, MarketDispute, MarketStatus, OutcomeReport},
    };
    use zrml_liquidity_mining::LiquidityMiningPalletApi;
    use zrml_market_commons::MarketCommonsPalletApi;

    type BalanceOf<T> =
        <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub(crate) type CurrencyOf<T> =
        <<T as Config>::MarketCommons as MarketCommonsPalletApi>::Currency;
    pub(crate) type MarketIdOf<T> =
        <<T as Config>::MarketCommons as MarketCommonsPalletApi>::MarketId;
    pub(crate) type MomentOf<T> = <<T as Config>::MarketCommons as MarketCommonsPalletApi>::Moment;

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Event
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Common market parameters
        type LiquidityMining: LiquidityMiningPalletApi<
            AccountId = Self::AccountId,
            Balance = BalanceOf<Self>,
            BlockNumber = Self::BlockNumber,
            MarketId = MarketIdOf<Self>,
        >;

        /// The identifier of individual markets.
        type MarketCommons: MarketCommonsPalletApi<
            AccountId = Self::AccountId,
            BlockNumber = Self::BlockNumber,
        >;

        /// The pallet identifier.
        type PalletId: Get<PalletId>;

        /// Swap shares
        type Shares: ZeitgeistMultiReservableCurrency<
            Self::AccountId,
            Balance = BalanceOf<Self>,
            CurrencyId = Asset<<Self::MarketCommons as MarketCommonsPalletApi>::MarketId>,
        >;

        /// Swaps pallet
        type Swaps: Swaps<Self::AccountId, Balance = BalanceOf<Self>, MarketId = MarketIdOf<Self>>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// 1. Any resolution must either have a `Disputed` or `Reported` market status
        /// 2. If status is `Disputed`, then at least one dispute must exist
        InvalidMarketStatus,
    }

    #[pallet::event]
    pub enum Event<T>
    where
        T: Config, {}

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    impl<T> DisputeApi for Pallet<T>
    where
        T: Config,
    {
        type AccountId = T::AccountId;
        type Balance = BalanceOf<T>;
        type BlockNumber = T::BlockNumber;
        type MarketId = MarketIdOf<T>;
        type Moment = MomentOf<T>;
        type Origin = T::Origin;

        fn on_dispute(
            _: &[MarketDispute<Self::AccountId, Self::BlockNumber>],
            _: &Self::MarketId,
        ) -> DispatchResult {
            Ok(())
        }

        fn on_resolution(
            disputes: &[MarketDispute<Self::AccountId, Self::BlockNumber>],
            _: &Self::MarketId,
            market: &Market<Self::AccountId, Self::BlockNumber, MomentOf<T>>,
        ) -> Result<OutcomeReport, DispatchError> {
            let report = T::MarketCommons::report(market)?;

            let resolved_outcome = match market.status {
                MarketStatus::Reported => report.outcome.clone(),
                MarketStatus::Disputed => {
                    // count the last dispute's outcome as the winning one
                    if let Some(last_dispute) = disputes.last() {
                        last_dispute.outcome.clone()
                    } else {
                        return Err(Error::<T>::InvalidMarketStatus.into());
                    }
                }
                _ => return Err(Error::<T>::InvalidMarketStatus.into()),
            };

            Ok(resolved_outcome)
        }
    }

    impl<T> SimpleDisputesPalletApi for Pallet<T> where T: Config {}

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);
}
