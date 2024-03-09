use crate::{mock::*, Config, Error, VoteType, Vote, VoteStatus, MembersCount};
use frame_support::{
	assert_noop, assert_ok,
	traits::{tokens::fungibles::{metadata::Inspect as MetadataInspect, Inspect}, OnFinalize, OnInitialize},
	BoundedVec,
};
use frame_system::RawOrigin;
use pallet_carbon_credits::{
	BatchGroupListOf, BatchGroupOf, BatchOf, ProjectCreateParams, RegistryListOf, SDGTypesListOf,
};
use primitives::{Batch, RegistryDetails, RegistryName, Royalty, SDGDetails, SdgType};
use sp_runtime::Percent;
use sp_std::convert::TryInto;

/// helper function to generate standard creation details

pub type ForestaCollectivesEvent = crate::Event<Test>;

/// helper function to generate standard registry details
fn get_default_registry_details<T: Config>() -> RegistryListOf<T> {
	let registry_details = RegistryDetails {
		reg_name: RegistryName::Verra,
		name: "reg_name".as_bytes().to_vec().try_into().unwrap(),
		id: "reg_id".as_bytes().to_vec().try_into().unwrap(),
		summary: "reg_summary".as_bytes().to_vec().try_into().unwrap(),
	};
	vec![registry_details].try_into().unwrap()
}

/// helper function to generate standard sdg details
fn get_default_sdg_details<T: Config>() -> SDGTypesListOf<T> {
	let sdg_details: SDGTypesListOf<T> = vec![SDGDetails {
		sdg_type: SdgType::LifeOnLand,
		description: "sdg_desp".as_bytes().to_vec().try_into().unwrap(),
		references: "sdg_ref".as_bytes().to_vec().try_into().unwrap(),
	}]
	.try_into()
	.unwrap();

	sdg_details
}

fn get_single_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![Batch {
		name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
		issuance_year: 2020_u16,
		start_date: 2020_u16,
		end_date: 2020_u16,
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
	}]
	.try_into()
	.unwrap()
}

fn get_multiple_batch_list<T: Config>() -> BoundedVec<BatchOf<T>, T::MaxGroupSize> {
	vec![
		Batch {
			name: "batch_name".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_uuid".as_bytes().to_vec().try_into().unwrap(),
			issuance_year: 2020_u16,
			start_date: 2020_u16,
			end_date: 2020_u16,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
		},
		Batch {
			name: "batch_name_2".as_bytes().to_vec().try_into().unwrap(),
			uuid: "batch_uuid_2".as_bytes().to_vec().try_into().unwrap(),
			issuance_year: 2021_u16,
			start_date: 2021_u16,
			end_date: 2021_u16,
			total_supply: 100_u32.into(),
			minted: 0_u32.into(),
			retired: 0_u32.into(),
		},
	]
	.try_into()
	.unwrap()
}

/// helper function to generate standard batch details
fn get_default_batch_group<T: Config>() -> BatchGroupListOf<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_single_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to generate multiple batch details
fn get_multiple_batch_group<T: Config>() -> BatchGroupListOf<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	vec![BatchGroupOf::<T> {
		name: "batch_group_name".as_bytes().to_vec().try_into().unwrap(),
		uuid: "batch_group_uuid".as_bytes().to_vec().try_into().unwrap(),
		asset_id: 0_u32.into(),
		total_supply: 100_u32.into(),
		minted: 0_u32.into(),
		retired: 0_u32.into(),
		batches: get_multiple_batch_list::<T>(),
	}]
	.try_into()
	.unwrap()
}

/// helper function to generate standard creation details
fn get_default_creation_params<T: Config>() -> ProjectCreateParams<T>
where
	<T as frame_system::Config>::AccountId: From<u32>,
{
	let royalty = Royalty::<T::AccountId> {
		account_id: 1_u32.into(),
		percent_of_fees: Percent::from_percent(0),
	};

	let creation_params = ProjectCreateParams {
		name: "name".as_bytes().to_vec().try_into().unwrap(),
		description: "description".as_bytes().to_vec().try_into().unwrap(),
		location: "(1, 1), (2, 2), (3, 3), (4, 4)".as_bytes().to_vec().try_into().unwrap(),
		images: vec!["image_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		videos: vec!["video_link".as_bytes().to_vec().try_into().unwrap()].try_into().unwrap(),
		documents: vec!["document_link".as_bytes().to_vec().try_into().unwrap()]
			.try_into()
			.unwrap(),
		registry_details: get_default_registry_details::<T>(),
		sdg_details: get_default_sdg_details::<T>(),
		royalties: Some(vec![royalty].try_into().unwrap()),
		batch_groups: get_default_batch_group::<T>(),
		project_type: None,
	};

	creation_params
}

pub fn create_project<T: Config>(
	originator_account: u64,
	batch: bool,
) {
	let mut creation_params = get_default_creation_params::<Test>();
	let project_id = 0;
	let group_id = 0;
	if batch {
		// replace the default with mutiple batches
		let created_batch_list = get_multiple_batch_group::<Test>();
		creation_params.batch_groups = created_batch_list;
	}

	let authorised_account = 10;

	assert_ok!(CarbonCredits::create(
		RawOrigin::Signed(originator_account).into(),
		creation_params
	));

}

fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			ForestaCollectives::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		ForestaCollectives::on_initialize(System::block_number());
	}
}


#[test]
fn it_works_for_add_collective() {
	new_test_ext().execute_with(|| {
		// Root creates collective
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![1],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));
	});
}

#[test]
fn it_fails_to_join_collective() {
	new_test_ext().execute_with(|| {
		let manager = 1;
		let collective_id = 0;
		// Root creates collective
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));
		assert_eq!(ForestaCollectives::get_membership_count(collective_id),1);

		let member = 4;// Not KYC'd KYCAuthorisationFailed

		assert_noop!(ForestaCollectives::join_collective(RawOrigin::Signed(member).into(),collective_id),
		Error::<Test>::KYCAuthorisationFailed);
	});
}

#[test]
fn it_works_for_adding_members_to_collective() {
	new_test_ext().execute_with(|| {
		let manager = 1;
		let collective_id = 0;
		// Root creates collective
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));
		assert_eq!(ForestaCollectives::get_membership_count(collective_id),1);

		let member = 2;

		assert_ok!(ForestaCollectives::join_collective(RawOrigin::Signed(member).into(),collective_id));
	});
}

#[test]
fn it_fails_to_add_members_to_collective() {
	new_test_ext().execute_with(|| {
		let manager = 1;
		let collective_id = 0;
		// Root creates collective
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));
		assert_eq!(ForestaCollectives::get_membership_count(collective_id),1);

		let member = 2;
		let not_collective_id = 1;

		assert_noop!(ForestaCollectives::join_collective(RawOrigin::Signed(member).into(),not_collective_id),
		Error::<Test>::CollectiveDoesNotExist);
	});
}


#[test]
fn it_works_for_add_collective_and_manager_adds_member() {
	new_test_ext().execute_with(|| {
		// Root creates collective
		let manager = 1;
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));

		let member = 2;
		let collective_id = 0;

		assert_ok!(ForestaCollectives::add_member(RawOrigin::Signed(manager).into(),collective_id,member));
	});
}

#[test]
fn it_fails_for_manager_adds_member() {
	new_test_ext().execute_with(|| {
		// Root creates collective
		let manager = 1;
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));

		let member = 2;
		let collective_id = 0;

		let not_manager = 3;

		assert_noop!(ForestaCollectives::add_member(RawOrigin::Signed(not_manager).into(),collective_id,member),
		Error::<Test>::NotAllowedToManageMembership);
	});
}

#[test]
fn it_works_for_join_collective() {
	new_test_ext().execute_with(|| {
		let collective_id = 0;
		let manager = 1;
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));

		let applicant = 2;

		assert_ok!(ForestaCollectives::join_collective(RawOrigin::Signed(applicant).into(),collective_id));
		assert_eq!(MembersCount::<Test>::get(collective_id),2);
	});
}

#[test]
fn it_works_for_init_project_approval_vote() {
	new_test_ext().execute_with(|| {
		let manager = 1;
		// Root creates collective and adds user 1 as the manager
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));

		let member = 2;
		let member2 = 3;
		let project_id = 0;
		let group_id = 0;
		let collective_id = 0;
		let vote_id = 0;
		let project_tokens_to_mint: u32 = 100;

		// Manager adds member as a member of the collective
		assert_ok!(ForestaCollectives::add_member(RawOrigin::Signed(manager).into(),collective_id,member));

		// Manager adds member2 as a member of the collective
		assert_ok!(ForestaCollectives::add_member(RawOrigin::Signed(manager).into(),collective_id,member2));
		// Manager creates a project

		create_project::<Test>(manager, false);

		// init project approval

		assert_ok!(ForestaCollectives::init_project_approval_removal(RawOrigin::Signed(member).into(),collective_id,
		project_id,VoteType::ProjectApproval));

		let mut vote = Vote::<Test> {
			yes_votes: 0,
			no_votes: 0,
			end: 101,
			status: VoteStatus::Deciding,
			vote_type: VoteType::ProjectApproval,
			collective_id: Some(collective_id),
			project_id: Some(project_id)
		};

		assert_eq!(ForestaCollectives::get_project_vote(vote_id),Some(vote));

		run_to_block(21);

		// member2 votes yes

		assert_ok!(ForestaCollectives::cast_vote(RawOrigin::Signed(member).into(),vote_id,true));

		vote = Vote::<Test> {
			yes_votes: 1,
			no_votes: 0,
			end: 101,
			status: VoteStatus::Deciding,
			vote_type: VoteType::ProjectApproval,
			collective_id: Some(collective_id),
			project_id: Some(project_id)
		};

		assert_eq!(ForestaCollectives::get_project_vote(vote_id),Some(vote));

		run_to_block(101);

		vote = Vote::<Test> {
			yes_votes: 1,
			no_votes: 0,
			end: 101,
			status: VoteStatus::Passed,
			vote_type: VoteType::ProjectApproval,
			collective_id: Some(collective_id),
			project_id: Some(project_id)
		};

		assert_eq!(ForestaCollectives::get_project_vote(vote_id),Some(vote));

		run_to_block(121);

		assert_ok!(CarbonCredits::mint(
			RawOrigin::Signed(manager).into(),
			project_id,
			group_id,
			project_tokens_to_mint.into(),
			false
		));

	});
}

#[test]
fn it_works_for_create_proposal() {
	new_test_ext().execute_with(|| {
		let manager = 1;
		// Root creates collective and adds user 1 as the manager
		assert_ok!(ForestaCollectives::add_collective(RawOrigin::Root.into(),"Collective1".as_bytes().to_vec().try_into().unwrap(),
		sp_core::bounded_vec![manager],"Coll1Hash".as_bytes().to_vec().try_into().unwrap()));

		let member = 2;
		let member2 = 3;
		let project_id = 0;
		let group_id = 0;
		let collective_id = 0;
		let vote_id = 0;

		// Manager adds member as a member of the collective
		assert_ok!(ForestaCollectives::add_member(RawOrigin::Signed(manager).into(),collective_id,member));

		// Manager adds member2 as a member of the collective
		assert_ok!(ForestaCollectives::add_member(RawOrigin::Signed(manager).into(),collective_id,member2));

		// member creates proposal
		assert_ok!(ForestaCollectives::create_proposal(RawOrigin::Signed(member).into(),collective_id,
		"Proposal1Hash".as_bytes().to_vec().try_into().unwrap()));

		let mut vote = Vote::<Test> {
			yes_votes: 0,
			no_votes: 0,
			end: 101,
			status: VoteStatus::Deciding,
			vote_type: VoteType::Proposal,
			collective_id: Some(collective_id),
			project_id: None
		};

		assert_eq!(ForestaCollectives::get_project_vote(vote_id),Some(vote));

		run_to_block(21);

		// member2 votes yes

		assert_ok!(ForestaCollectives::cast_vote(RawOrigin::Signed(member2).into(),vote_id,true));

		vote = Vote::<Test> {
			yes_votes: 1,
			no_votes: 0,
			end: 101,
			status: VoteStatus::Deciding,
			vote_type: VoteType::Proposal,
			collective_id: Some(collective_id),
			project_id: None
		};

		assert_eq!(ForestaCollectives::get_project_vote(vote_id),Some(vote));

		run_to_block(101);

		vote = Vote::<Test> {
			yes_votes: 1,
			no_votes: 0,
			end: 101,
			status: VoteStatus::Passed,
			vote_type: VoteType::Proposal,
			collective_id: Some(collective_id),
			project_id: None
		};

		assert_eq!(ForestaCollectives::get_project_vote(vote_id),Some(vote));

		//member tries to vote after vote ahs expired

		run_to_block(121);

		assert_noop!(ForestaCollectives::cast_vote(RawOrigin::Signed(member).into(),vote_id,true),
	Error::<Test>::VoteNotInProgress);
	});
}
