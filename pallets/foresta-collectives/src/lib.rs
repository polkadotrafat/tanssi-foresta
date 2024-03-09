#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{BoundedVec,PalletId};
	use scale_info::TypeInfo;
	use codec::{FullCodec, MaxEncodedLen, EncodeLike};

	use sp_runtime::{
		traits::{MaybeSerializeDeserialize,CheckedAdd,AccountIdConversion}
		,ArithmeticError, };
	use sp_std::{fmt::Debug,cmp::{Eq, PartialEq}};
	use frame_support::traits::Contains;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Collective<T:Config> {
        pub name: BoundedVec<u8,T::MaxStringLength>,
		pub hash: BoundedVec<u8, T::MaxStringLength>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Vote<T:Config> {
		pub yes_votes: u64,
		pub no_votes: u64,
		pub end: BlockNumberFor<T>,
		pub status: VoteStatus,
		pub vote_type: VoteType,
		pub collective_id: Option<T::CollectiveId>,
		pub project_id: Option<<T as pallet_carbon_credits::Config>::ProjectId>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct Proposal<T:Config> {
        pub creator: T::AccountId,
		pub hash: BoundedVec<u8, T::MaxStringLength>,
		pub vote_id: T::VoteId,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct CCParams<T:Config> {
		pub project_details: pallet_carbon_credits::ProjectDetail<T>,
		pub item_id: <T as pallet_carbon_credits::Config>::ItemId,
		pub asset_id: <T as pallet_carbon_credits::Config>::AssetId,
		pub retired_carbon_credits_data: pallet_carbon_credits::RetiredCarbonCreditsData<T>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct PoolParams<T:Config> {
		id: <T as pallet_carbon_credits_pool::Config>::PoolId,
		admin: T::AccountId,
		config: pallet_carbon_credits_pool::PoolConfigOf<T>,
		max_limit: Option<u32>,
		asset_symbol: pallet_carbon_credits_pool::SymbolStringOf<T>,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen, Debug, TypeInfo, Eq)]
	#[scale_info(skip_type_params(T))]
	pub struct DexParams<T:Config> {		
		account: T::AccountId,
	}
	
	#[derive(Clone, Encode, Decode, PartialEq, Debug,MaxEncodedLen, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum MembershipStatus {
		Active,
		InActive,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen,Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum VoteStatus {
		Deciding,
		Passed,
		Failed,
	}

	#[derive(Clone, Encode, Decode, PartialEq, MaxEncodedLen,Debug, TypeInfo, Eq, Copy)]
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	pub enum VoteType {
		ProjectApproval,
		ProjectRemoval,
		PoolCreation,
		AddValidator,
		RemoveValidator,
		SetSellerPayoutAuthority,
		Proposal,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_carbon_credits::Config +
	 pallet_carbon_credits_pool::Config + pallet_dex::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		type KYCProvider: Contains<Self::AccountId>;
		type CollectiveId: Parameter
			+ FullCodec
			+ Default
			+ Eq
			+ PartialEq
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Debug
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ EncodeLike
			+ CheckedAdd;
		type VoteId: Parameter
			+ FullCodec
			+ Default
			+ Eq
			+ PartialEq
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Debug
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ EncodeLike
			+ CheckedAdd;
		type ProposalId: Parameter
			+ FullCodec
			+ Default
			+ Eq
			+ PartialEq
			+ Copy
			+ MaxEncodedLen
			+ MaybeSerializeDeserialize
			+ Debug
			+ TypeInfo
			+ From<u32>
			+ Into<u32>
			+ EncodeLike
			+ CheckedAdd;
		/// The ForestaCollectives pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type MaxStringLength: Get<u32>;
		type MaxNumManagers: Get<u32>;
		type MaxConcurrentVotes: Get<u32>;
		type MaxProjectsPerCollective: Get<u32>;
		type VotingDuration: Get<BlockNumberFor<Self>>;
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;


	#[pallet::storage]
	#[pallet::getter(fn get_collective)]
	pub(super) type CollectivesMap<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		Collective<T>,
		OptionQuery,
	>;


	#[pallet::storage]
	#[pallet::getter(fn check_member)]
	pub(super) type Members<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_proposal)]
	pub(super) type Proposals<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		Blake2_128Concat,
		T::ProposalId,
		Proposal<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_membership_count)]
	pub(super) type MembersCount<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		u32,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_proposal_count)]
	pub(super) type ProposalsCount<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		T::ProposalId,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_managers)]
	pub(super) type Managers<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		BoundedVec<T::AccountId, T::MaxNumManagers>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_approved_projects)]
	pub(super) type ApprovedProjects<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CollectiveId,
		BoundedVec<<T as pallet_carbon_credits::Config>::ProjectId, T::MaxProjectsPerCollective>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_vote_by_id)]
	pub(super) type Votes<T:Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::VoteId,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_project_voting)]
	pub type ActiveVoting<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<T::VoteId, T::MaxConcurrentVotes>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_project_vote)]
	pub(super) type ProjectVote<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::VoteId,
		Vote<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_pool_params_info)]
	pub(super) type PoolParamsInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::VoteId,
		PoolParams<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_dex_params_info)]
	pub(super) type DexParamsInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::VoteId,
		DexParams<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn check_member_vote)]
	pub(super) type CheckMemberVote<T:Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::VoteId,
		bool,
		ValueQuery,
	>;


	#[pallet::storage]
	#[pallet::getter(fn collectives_count)]
	pub(super) type CollectivesCount<T: Config> = StorageValue<_, T::CollectiveId,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn votes_count)]
	pub type VotesCount<T: Config> = StorageValue<_, T::VoteId, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		CollectiveCreated { uid : T::CollectiveId },
		MemberAdded {collective_id: T::CollectiveId, member: T::AccountId, uid: u32},
		ProjectApprovalInit { collective_id: T::CollectiveId, project_id: <T as pallet_carbon_credits::Config>::ProjectId},
		ProjectApprovalVoteCast { collective_id: T::CollectiveId, project_id: <T as pallet_carbon_credits::Config>::ProjectId},
		ProjectApprovalRemovalInit { collective_id: T::CollectiveId, project_id: <T as pallet_carbon_credits::Config>::ProjectId},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// KYC Failed
		KYCAuthorisationFailed,
		/// Member Already Exists
		MemberAlreadyExists,
		/// Member Does Not Exist
		MemberDoesNotExist,
		/// Collective Does Not exist
		CollectiveDoesNotExist,
		/// Not Allowed To Manage Membership
		NotAllowedToManageMembership,
		/// NoManagersFound
		NoManagersFound,
		/// MismatchTypes
		MisMatchTypes,
		/// Max Voting Exceeded
		MaxVotingExceeded,
		/// Vote Not Found
		VoteNotFound,
		/// Not Allowed To Vote
		NotAllowedToVote,
		/// Vote Not In Progress
		VoteNotInProgress,
		/// Already Voted
		AlreadyVoted,
		/// Project Not Found
		ProjectNotFound,
		/// Max Projects Exceeded
		MaxProjectsExceeded,
		/// Wrong Vote Type
		WrongVoteType,
		/// Params Not Found
		ParamsNotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 1);

			let approval = ActiveVoting::<T>::take(n);

			for v_id in approval.iter() {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(2, 2));
				let vote = ProjectVote::<T>::take(v_id);

				if let Some(mut vote) = vote {
					let mut is_approved: bool = false;
					if vote.yes_votes > vote.no_votes {
						vote.status = VoteStatus::Passed;
						is_approved = true;						
					} else {
						vote.status = VoteStatus::Failed;						
					}

					match vote.vote_type {
						VoteType::ProjectApproval => {
							let _ = Self::do_approve_project(vote.collective_id,vote.project_id,is_approved);
						},
						VoteType::ProjectRemoval => {
							let _ = Self::do_remove_project(vote.collective_id,vote.project_id,is_approved);	
						},
						VoteType::PoolCreation => {
							let _ = Self::do_create_pool(*v_id,is_approved);	
						},
						VoteType::AddValidator => {
							let _ = Self::do_add_validator(*v_id,is_approved);	
						},
						VoteType::RemoveValidator => {
							let _ = Self::do_remove_validator(*v_id,is_approved);	
						},
						VoteType::SetSellerPayoutAuthority=> {
							let _ = Self::do_add_validator(*v_id,is_approved);	
						},
						_ => ()
					}

					ProjectVote::<T>::insert(v_id,&vote);

				}

			}

			weight
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn add_collective(origin: OriginFor<T>, name: BoundedVec<u8,T::MaxStringLength>,
		managers: BoundedVec<T::AccountId, T::MaxNumManagers>,hash : BoundedVec<u8,T::MaxStringLength>)  -> DispatchResult {
			
			<T as pallet::Config>::ForceOrigin::ensure_origin(origin)?;

			let collective = Collective::<T> {
				name: name,
				hash: hash,
			};

			let uid = Self::collectives_count();
			let uid2 = uid.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;
			CollectivesMap::<T>::insert(uid.clone(),&collective);
			Managers::<T>::insert(uid.clone(),&managers);
			
			// Add managers to authorized accounts
			let mut mid = Self::get_membership_count(uid.clone());

			for manager in managers {
				let _ = Self::do_authorize_account(manager.clone());
				mid = mid.checked_add(1).ok_or(ArithmeticError::Overflow)?;
				Members::<T>::insert(uid.clone(),manager.clone(),true);
				MembersCount::<T>::insert(uid.clone(),mid);
			}
			
			CollectivesCount::<T>::put(uid2);
			Self::deposit_event(Event::CollectiveCreated{ uid });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn join_collective(origin: OriginFor<T>, collective_id: T::CollectiveId) -> DispatchResult {
			let member = ensure_signed(origin)?;
			Self::check_kyc_approval(&member)?;
			ensure!(!Self::check_member(collective_id,member.clone()),Error::<T>::MemberAlreadyExists);
			ensure!(Self::get_collective(collective_id).is_some(),Error::<T>::CollectiveDoesNotExist);
			let uid = Self::get_membership_count(collective_id.clone());
			let uid2 = uid.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			Members::<T>::insert(collective_id.clone(),member.clone(),true);
			MembersCount::<T>::insert(collective_id.clone(),uid2);
			
			Self::deposit_event(Event::MemberAdded{ collective_id, member, uid });

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_member())]
		pub fn add_member(origin: OriginFor<T>, collective_id: T::CollectiveId, member: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(Self::get_collective(collective_id).is_some(),Error::<T>::CollectiveDoesNotExist);
			ensure!(!Members::<T>::contains_key(collective_id.clone(),&member.clone()), Error::<T>::MemberAlreadyExists);
			let managers = Managers::<T>::get(collective_id.clone());
			
			match managers.binary_search(&who) {
				Ok(_) => {
					let uid = Self::get_membership_count(collective_id.clone());
					let uid2 = uid.checked_add(1).ok_or(ArithmeticError::Overflow)?;
					Members::<T>::insert(collective_id.clone(),member.clone(),true);
					MembersCount::<T>::insert(collective_id.clone(),uid2);
			
					Self::deposit_event(Event::MemberAdded{ collective_id, member, uid });

					Ok(())
				},
				Err(_) => Err(Error::<T>::NotAllowedToManageMembership.into()),

			}
			
		}

		#[pallet::call_index(3)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn init_project_approval_removal(origin: OriginFor<T>, collective_id: T::CollectiveId, 
		project_id: <T as pallet_carbon_credits::Config>::ProjectId, vote_type: VoteType) -> DispatchResult {
			
			let who = ensure_signed(origin)?;
			ensure!(Self::get_collective(collective_id).is_some(),Error::<T>::CollectiveDoesNotExist);
			ensure!(Members::<T>::contains_key(collective_id.clone(),&who.clone()), Error::<T>::MemberDoesNotExist);
			ensure!(vote_type == VoteType::ProjectApproval || vote_type == VoteType::ProjectRemoval,
				Error::<T>::WrongVoteType);

			let _project_details: pallet_carbon_credits::ProjectDetail<T>;
			
			if vote_type == VoteType::ProjectRemoval {
				_project_details = pallet_carbon_credits::Pallet::get_project_details(project_id)
						.ok_or(Error::<T>::ProjectNotFound)?;
			}
			let uid = Self::votes_count();
			
			let current_block = <frame_system::Pallet<T>>::block_number();

			let final_block = current_block + T::VotingDuration::get();

			ActiveVoting::<T>::try_mutate(final_block, |projects| {
				projects.try_push(uid).map_err(|_| Error::<T>::MaxVotingExceeded)?;
				Ok::<(),DispatchError>(())
			})?; 

			let vote_info = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				end: final_block,
				status: VoteStatus::Deciding,
				vote_type: VoteType::ProjectApproval,
				collective_id: Some(collective_id),
				project_id: Some(project_id),
			};

			ProjectVote::<T>::insert(uid,&vote_info);
			let uid2 = uid.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;
			VotesCount::<T>::put(uid2);

			Self::deposit_event(Event::ProjectApprovalRemovalInit{ collective_id, project_id });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn cast_vote(origin: OriginFor<T>,vote_id: T::VoteId, 
		vote_cast: bool) -> DispatchResult {
			let who = ensure_signed(origin)?;
			// Get vote
			let mut vote = Self::get_project_vote(vote_id).ok_or(Error::<T>::VoteNotFound)?;

			match vote.collective_id {
				Some(x) => ensure!(Members::<T>::contains_key(x,who.clone()), Error::<T>::NotAllowedToVote),
				None => Self::check_kyc_approval(&who)?,
			}
			// Check if member
			
			
			
			// Check if vote is in progress
			ensure!(vote.status == VoteStatus::Deciding, Error::<T>::VoteNotInProgress);
			// Check if member has already voted
			ensure!(!Self::check_member_vote(who.clone(),vote_id), Error::<T>::AlreadyVoted);

			if vote_cast {
				vote.yes_votes = vote.yes_votes + 1;
			} else {
				vote.no_votes = vote.no_votes + 1;
			}

			ProjectVote::<T>::insert(vote_id,vote);
			CheckMemberVote::<T>::insert(who.clone(),vote_id,true);

			//Self::deposit_event(Event::ProjectApprovalVoteCast{ collective_id, project_id });
			
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn init_create_pool(origin: OriginFor<T>, id: <T as pallet_carbon_credits_pool::Config>::PoolId,
			admin: T::AccountId,
			config: pallet_carbon_credits_pool::PoolConfigOf<T>,
			max_limit: Option<u32>,
			asset_symbol: pallet_carbon_credits_pool::SymbolStringOf<T>) -> DispatchResult {

			let member = ensure_signed(origin)?;
			// Check if member
			Self::check_kyc_approval(&member)?;

			let uid = Self::votes_count();
			
			let current_block = <frame_system::Pallet<T>>::block_number();

			let final_block = current_block + T::VotingDuration::get();

			ActiveVoting::<T>::try_mutate(final_block, |projects| {
				projects.try_push(uid).map_err(|_| Error::<T>::MaxVotingExceeded)?;
				Ok::<(),DispatchError>(())
			})?; 

			let vote_info = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				end: final_block,
				status: VoteStatus::Deciding,
				vote_type: VoteType::PoolCreation,
				collective_id: None,
				project_id: None,
			};

			ProjectVote::<T>::insert(uid,&vote_info);

			let pool_params_info = PoolParams::<T> {
				id: id,
				admin: admin,
				config: config,
				max_limit: max_limit,
				asset_symbol: asset_symbol,
			};

			PoolParamsInfo::<T>::insert(uid,&pool_params_info);
			let uid2 = uid.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;
			VotesCount::<T>::put(uid2);


			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn init_dex_management_vote(origin: OriginFor<T>, account: T::AccountId, vote_type: VoteType) -> DispatchResult {
			let member = ensure_signed(origin)?;
			// Check if member
			Self::check_kyc_approval(&member)?;
			ensure!(vote_type == VoteType::AddValidator || vote_type == VoteType::RemoveValidator || 
			vote_type == VoteType::SetSellerPayoutAuthority, Error::<T>::WrongVoteType);

			let uid = Self::votes_count();
			let current_block = <frame_system::Pallet<T>>::block_number();

			let final_block = current_block + T::VotingDuration::get();

			ActiveVoting::<T>::try_mutate(final_block, |projects| {
				projects.try_push(uid).map_err(|_| Error::<T>::MaxVotingExceeded)?;
				Ok::<(),DispatchError>(())
			})?; 

			let vote_info = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				end: final_block,
				status: VoteStatus::Deciding,
				vote_type: vote_type,
				collective_id: None,
				project_id: None,
			};

			let dex_params_info = DexParams::<T> {
				account: account,
			};

			ProjectVote::<T>::insert(uid,&vote_info);
			DexParamsInfo::<T>::insert(uid,&dex_params_info);
			let uid2 = uid.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;
			VotesCount::<T>::put(uid2);

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::add_collective())]
		pub fn create_proposal(origin: OriginFor<T>, collective_id: T::CollectiveId,
			proposal_hash: BoundedVec<u8, T::MaxStringLength>) -> DispatchResult {
			
			let who = ensure_signed(origin)?;
			ensure!(Self::get_collective(collective_id).is_some(),Error::<T>::CollectiveDoesNotExist);
			ensure!(Members::<T>::contains_key(collective_id.clone(),&who.clone()), Error::<T>::MemberDoesNotExist);

			let uid = Self::votes_count();
		
			let current_block = <frame_system::Pallet<T>>::block_number();

			let final_block = current_block + T::VotingDuration::get();

			ActiveVoting::<T>::try_mutate(final_block, |projects| {
				projects.try_push(uid).map_err(|_| Error::<T>::MaxVotingExceeded)?;
				Ok::<(),DispatchError>(())
			})?;

			let vote_info = Vote::<T> {
				yes_votes: 0,
				no_votes: 0,
				end: final_block,
				status: VoteStatus::Deciding,
				vote_type: VoteType::Proposal,
				collective_id: Some(collective_id),
				project_id: None,
			};

			let proposal_info = Proposal::<T> {
				creator: who.clone(),
				hash: proposal_hash,
				vote_id: uid,
			};

			let proposal_count = Self::get_proposal_count(collective_id);


			ProjectVote::<T>::insert(uid,&vote_info);
			Proposals::<T>::insert(collective_id,proposal_count,&proposal_info);

			let uid2 = uid.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;
			VotesCount::<T>::put(uid2);

			let proposal_count2 = proposal_count.checked_add(&1u32.into()).ok_or(ArithmeticError::Overflow)?;

			ProposalsCount::<T>::insert(collective_id,proposal_count2);


			Ok(())
		}

	}

	impl<T:Config> Pallet<T> {
		pub fn check_kyc_approval(account_id: &T::AccountId) -> DispatchResult {
			if !<T as pallet::Config>::KYCProvider::contains(account_id) {
				Err(Error::<T>::KYCAuthorisationFailed.into())
			} else {
				Ok(())
			}
		}

		pub fn do_approve_project(coll_id: Option<T::CollectiveId>,
		proj_id: Option<<T as pallet_carbon_credits::Config>::ProjectId>, is_approved: bool) -> DispatchResult {
				
			if is_approved == true {
				let collective_id = match coll_id {
					Some(x) => x,
					None => 0.into(),
				};
	
				let project_id = match proj_id {
					Some(x) => x,
					None => 0.into(),
				};
	
				pallet_carbon_credits::Pallet::<T>::approve_project(frame_system::RawOrigin::Root.into(),project_id,is_approved)?;
				ApprovedProjects::<T>::try_mutate(collective_id, |projects| {
					projects.try_push(project_id).map_err(|_| Error::<T>::MaxProjectsExceeded)?;
					Ok::<(),DispatchError>(())
				})?; 

				// Add project originator to approved accounts

				let project_details: pallet_carbon_credits::ProjectDetail<T> = pallet_carbon_credits::Pallet::get_project_details(project_id)
				.ok_or(Error::<T>::ProjectNotFound)?;

				let _ = Self::do_authorize_account(project_details.originator)?;
				
			}
			Ok(())
		}

		pub fn do_remove_project(coll_id: Option<T::CollectiveId>,
		proj_id: Option<<T as pallet_carbon_credits::Config>::ProjectId>, is_approved: bool) -> DispatchResult {
				if is_approved == true {
					let collective_id = match coll_id {
						Some(x) => x,
						None => 0.into(),
					};
		
					let project_id = match proj_id {
						Some(x) => x,
						None => 0.into(),
					};
					pallet_carbon_credits::Pallet::<T>::force_remove_project(frame_system::RawOrigin::Root.into(),project_id)?;
					let mut projects = ApprovedProjects::<T>::get(collective_id);

					projects.retain(|x| *x != project_id);

				}

				Ok(())
		}

		pub fn do_create_pool(vote_id: T::VoteId, is_approved: bool) -> DispatchResult {
			if is_approved == true {
				let params = Self::get_pool_params_info(vote_id).ok_or(Error::<T>::ParamsNotFound)?;
				let _ = pallet_carbon_credits_pool::Pallet::<T>::create(frame_system::RawOrigin::Root.into(),
				params.id,params.admin,params.config,params.max_limit,params.asset_symbol);
					
			}

			Ok(())
		}

		pub fn do_add_validator(vote_id: T::VoteId, is_approved: bool) -> DispatchResult {
			if is_approved == true {
				let params = Self::get_dex_params_info(vote_id).ok_or(Error::<T>::ParamsNotFound)?;
				let _ = pallet_dex::Pallet::<T>::force_add_validator_account(frame_system::RawOrigin::Root.into(),params.account);
			}

			Ok(())
			
		}

		pub fn do_remove_validator(vote_id: T::VoteId, is_approved: bool) -> DispatchResult {
			if is_approved == true {
				let params = Self::get_dex_params_info(vote_id).ok_or(Error::<T>::ParamsNotFound)?;
				let _ = pallet_dex::Pallet::<T>::force_remove_validator_account(frame_system::RawOrigin::Root.into(),params.account);
			}

			Ok(())
			
		}

		pub fn do_set_seller_payout_authority(vote_id: T::VoteId, is_approved: bool) -> DispatchResult {
			if is_approved == true {
				let params = Self::get_dex_params_info(vote_id).ok_or(Error::<T>::ParamsNotFound)?;
				let _ = pallet_dex::Pallet::<T>::force_set_seller_payout_authority(frame_system::RawOrigin::Root.into(),params.account);
			}

			Ok(())
		}

		pub fn do_authorize_account(account: T::AccountId) -> DispatchResult {
			if !Self::check_authorized_account(&account.clone()) {
				pallet_carbon_credits::Pallet::<T>::force_add_authorized_account(frame_system::RawOrigin::Root.into(),account)?;
			}
			
			Ok(())
		}

		pub fn check_authorized_account(account_id: &T::AccountId) -> bool {
			let authorized_accounts = pallet_carbon_credits::AuthorizedAccounts::<T>::get();
			if authorized_accounts.contains(account_id) {
				true
			} else {
				false
			}
		}

		/// The account ID of the ForestaCollectives pallet
		pub fn account_id() -> T::AccountId {
			<T as pallet::Config>::PalletId::get().into_account_truncating()
		}

	}
}

