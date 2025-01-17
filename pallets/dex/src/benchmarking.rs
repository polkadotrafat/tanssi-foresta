// This file is part of BitGreen.
// Copyright (C) 2022 BitGreen.
// This code is licensed under MIT license (see LICENSE.txt for details)
//! Dex pallet benchmarking
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::{RawOrigin, RawOrigin as SystemOrigin};
use orml_traits::MultiCurrency;
use primitives::CurrencyId;
use sp_runtime::traits::StaticLookup;
use sp_std::prelude::*;

use super::*;
use crate::Pallet as Dex;
use sp_runtime::Percent;

fn get_currency_id() -> CurrencyId {
	primitives::CurrencyId::USDT
}

fn create_default_asset<T: Config + pallet_assets::Config>(
	is_sufficient: bool,
) -> (T::AccountId, <T::Lookup as StaticLookup>::Source) {
	let caller: T::AccountId = whitelisted_caller();
	let caller_lookup = T::Lookup::unlookup(caller.clone());
	let root = SystemOrigin::Root.into();
	assert!(<pallet_assets::Pallet<T>>::force_create(
		root,
		Default::default(),
		caller_lookup.clone(),
		is_sufficient,
		1u32.into(),
	)
	.is_ok());
	(caller, caller_lookup)
}

fn create_default_minted_asset<T: Config + pallet_assets::Config>(
	is_sufficient: bool,
	amount: <T as pallet_assets::Config>::Balance,
) -> (T::AccountId, <T::Lookup as StaticLookup>::Source) {
	let (caller, caller_lookup) = create_default_asset::<T>(is_sufficient);

	assert!(<pallet_assets::Pallet<T>>::mint(
		SystemOrigin::Signed(caller.clone()).into(),
		Default::default(),
		caller_lookup.clone(),
		amount,
	)
	.is_ok());
	(caller, caller_lookup)
}

benchmarks! {

	where_clause { where
		T: pallet_assets::Config,
		T: orml_tokens::Config,
		<<T as pallet::Config>::Asset as frame_support::traits::fungibles::Inspect<<T as frame_system::Config>::AccountId>>::AssetId : From<u32>,
		T: orml_tokens::Config<CurrencyId = CurrencyId>,
	}

	create_sell_order {
		create_default_minted_asset::<T>(true, 100u32.into());
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.into()), 0u32.into(), 100u32.into(), 10u32.into())
	verify {
		assert!(Orders::<T>::get(0u128).is_some())
	}

	cancel_sell_order {
		create_default_minted_asset::<T>(true, 100u32.into());
		let caller: T::AccountId = whitelisted_caller();
		Dex::<T>::create_sell_order(RawOrigin::Signed(caller.clone()).into(), 0u32.into(), 100u32.into(), 10u32.into())?;
	}: _(RawOrigin::Signed(caller.into()), 0u128)
	verify {
		assert!(Orders::<T>::get(0u128).is_none())
	}

	create_buy_order {
		create_default_minted_asset::<T>(true, 100u32.into());
		let caller: T::AccountId = whitelisted_caller();
		Dex::<T>::create_sell_order(RawOrigin::Signed(caller.clone()).into(), 0u32.into(), 100u32.into(), 1u32.into())?;
		// give the buyer some tokens to pay
		let buyer : T::AccountId = account("account_id", 0, 1);
		<orml_tokens::Pallet<T>>::deposit(get_currency_id(), &buyer, 1000u32.into())?;
	}: _(RawOrigin::Signed(buyer.into()), 0u128, 0u32.into(), 1u32.into(), 100u32.into())
	verify {}


	force_set_payment_fee {
	}: _(RawOrigin::Root, Percent::from_percent(10))
	verify {
		assert_eq!(PaymentFees::<T>::get(), Percent::from_percent(10));
	}

	force_set_purchase_fee {
	}: _(RawOrigin::Root, 10u32.into())
	verify {
		assert_eq!(PurchaseFees::<T>::get(), 10u32.into());
	}


	impl_benchmark_test_suite!(Dex, crate::mock::new_test_ext(), crate::mock::Test);
}