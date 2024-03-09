//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::Pallet as ForestaCollectives;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_std::vec;
use pallet_carbon_credits::{BatchGroupOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf, BatchOf, BatchGroupListOf};
use primitives::{Batch, RegistryDetails, RegistryName, SDGDetails, SdgType};
use frame_support::BoundedVec;
use sp_std::convert::TryInto;


benchmarks! {
	where_clause { where
		T: pallet_membership::Config
	}
	add_collective {
		let manager : T::AccountId = account("account_id", 0, 0);
		let collective_id : T::CollectiveId = 0_u32.into();
	}: _(RawOrigin::Root, "Collective1".as_bytes().to_vec().try_into().unwrap(),
	vec![manager].try_into().unwrap(),"Coll1Hash".as_bytes().to_vec().try_into().unwrap())
	verify {
		assert!(CollectivesMap::<T>::get(collective_id).is_some());
	}
/*
	join_collective {
		let manager : T::AccountId = account("account_id", 0, 1);
		let collective_id : T::CollectiveId = 0_u32.into();
		let member : T::AccountId = account("account_id", 0, 0);
		let member_lookup = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(member.clone());
		pallet_membership::Pallet::<T>::add_member(RawOrigin::Root.into(), member_lookup)?;
		let _ = ForestaCollectives::<T>::add_collective(RawOrigin::Root.into(), "Collective1".as_bytes().to_vec().try_into().unwrap(),
		vec![manager].try_into().unwrap(),"Coll1Hash".as_bytes().to_vec().try_into().unwrap());
	}: _(RawOrigin::Signed(member.into()),collective_id)
	verify {
		assert_eq!(MembersCount::<T>::get(collective_id),2_u32);
	}
*/
	add_member {
		let manager : T::AccountId = account("account_id", 0, 1);
		let collective_id : T::CollectiveId = 0_u32.into();
		let member : T::AccountId = account("account_id", 0, 0);
		let _ = ForestaCollectives::<T>::add_collective(RawOrigin::Root.into(), "Collective1".as_bytes().to_vec().try_into().unwrap(),
		vec![manager.clone()].try_into().unwrap(),"Coll1Hash".as_bytes().to_vec().try_into().unwrap());
	}: _(RawOrigin::Signed(manager.clone()),collective_id, member)
	verify {
		assert_eq!(MembersCount::<T>::get(collective_id),2_u32);
	}


	impl_benchmark_test_suite!(ForestaCollectives, crate::mock::new_test_ext(), crate::mock::Test);
}

