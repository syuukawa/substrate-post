/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, 
			StorageValue, StorageMap, Parameter, dispatch::Result, ensure};
use sr_primitives::traits::{SimpleArithmetic, Bounded, Member};
// use codec::{Encode, Decode};
use system::ensure_signed;
use rstd::result;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	/// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	///role_index Manage(0) Operator(1) Receiver(2)
	type RoleIndex: Parameter + Member + SimpleArithmetic + Bounded + Default + Copy;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as PostRole {

		///-----------------------Manager-----------------------
		/// 数量
		/// Stores the total number of manage. i.e. the next manger index
		/// the count of the manage
		pub ManagerCount get(manager_count): T::RoleIndex;

		/// 列表
		/// Stores all the Parcel, key is the parcel id/index
		pub Managers get(managers): map T::RoleIndex => Option<T::AccountId>;

		/// Get manager RoleIndex
		// pub ManagerOwners get(manager_owner): map T::RoleIndex => Option<T::AccountId>;
		pub ManagerToIndex get(manager_to_index): map T::AccountId => Option<T::RoleIndex>;
		
		/// creator and the manager list
		pub CreatorMangerList get(creator_manager_list): map (T::AccountId, T::AccountId) => Option<T::RoleIndex>;

		///-----------------------operator-----------------------
		/// 数量
		/// Stores the total number of operator. i.e. the next operator index
		/// the count of the operator
		pub OperatorCount get(operator_count): T::RoleIndex;

		/// 列表
		/// Stores all the Operators, key is the parcel id/index
		pub Operators get(operators): map T::RoleIndex => Option<T::AccountId>;

		/// Get OperatorToIndex RoleIndex
		// pub ManagerOwners get(manager_owner): map T::RoleIndex => Option<T::AccountId>;
		pub OperatorToIndex get(operator_to_index): map T::AccountId => Option<T::RoleIndex>;
		
		/// creator and the manager list
		pub ManagerOperatorList get(manager_operator_list): map (T::AccountId, T::AccountId) => Option<T::RoleIndex>;

	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		/// add new manage to PostManagerList
 		pub fn add_post_manage_list(origin, account_id:T::AccountId) ->Result {

			// check it was signed.
			let sender = ensure_signed(origin)?;

			// Create and store kitty
			let old_manage_index = Self::next_manager_index()?;
			let new_manage_index = old_manage_index + 1.into();
			<ManagerCount<T>>::put(new_manage_index);
			<Managers<T>>::insert(new_manage_index, account_id.clone());
			// <ManagerOwners<T>>::insert(new_manage_index, account_id.clone());
			<ManagerToIndex<T>>::insert(account_id.clone(),new_manage_index);
			<CreatorMangerList<T>>::insert((sender.clone(),account_id.clone()),new_manage_index);

			// AddRoleList event
			Self::deposit_event(RawEvent::AddRoleList(sender, account_id, new_manage_index));

			Ok(())
		}

		/// remove the account_id  from the manager list 
 		pub fn remove_manage_list(origin, account_id:T::AccountId) ->Result {

			/// check it was signed.
			let sender = ensure_signed(origin)?;

			/// check the account_id whether included in the is_manager_map
			let _manager_index = Self::manager_to_index(&account_id);
			ensure!(_manager_index.is_some() , "_manager_index: Not the manage of the post");

			let _creator_manager_index = Self::creator_manager_list((sender.clone(),account_id.clone()));
			ensure!(_creator_manager_index.is_some() ,"_creator_manager_index: Not the manage of the post");
			
			// mod the count of the manager
			let old_manage_index = Self::next_manager_index()?;
			let new_manage_index = old_manage_index - 1.into();
			<ManagerCount<T>>::put(new_manage_index);

			// let Some(_remove_manger_index) = _manager_index;
			<Managers<T>>::remove(_manager_index.unwrap()); 
			// <ManagerOwners<T>>::insert(new_manage_index, account_id.clone());
			<ManagerToIndex<T>>::remove(account_id.clone());
			<CreatorMangerList<T>>::remove((sender.clone(),account_id.clone()));
			
			// RemoveRoleList event
			Self::deposit_event(RawEvent::RemoveRoleList(sender, account_id));

			Ok(())
		}

		/// add new operator to add_post_operator_list
		/// origin: Manager 
		/// account_id : Auth the account_id to operator
 		pub fn add_post_operator_list(origin, account_id:T::AccountId) ->Result {

			/// check it was signed.
			let sender = ensure_signed(origin)?;
			ensure!(sender != account_id , "manager and operator is the same account");

			/// check the account_id whether included in the is_manager_map
			let _operator_index = Self::manager_to_index(&sender);
			ensure!(_operator_index.is_some(), "_manager_index: Not the manage");
		
			let old_operator_index = Self::next_operator_index()?;
			let new_operator_index = old_operator_index + 1.into();
			<OperatorCount<T>>::put(new_operator_index);

			<Operators<T>>::insert(new_operator_index, account_id.clone());
			// <ManagerOwners<T>>::insert(new_manage_index, account_id.clone());
			<OperatorToIndex<T>>::insert(account_id.clone(),new_operator_index);
			<ManagerOperatorList<T>>::insert((sender.clone(),account_id.clone()),new_operator_index);
			
			// AddRoleList event
			Self::deposit_event(RawEvent::AddRoleList(sender, account_id, new_operator_index));

			Ok(())
		}

		/// remove operator from operatorList
		/// origin: Manager 
		/// account_id : Auth the account_id to operator
 		pub fn remove_post_operator_list(origin, account_id:T::AccountId) ->Result {

			/// check it was signed.
			let sender = ensure_signed(origin)?;

			/// check the account_id whether included in the is_manager_map
			let _operator_index = Self::manager_to_index(&sender);
			ensure!(_operator_index.is_some(), "is_manager: Not the manage");

			let _manger_operator_index = Self::manager_operator_list((sender.clone(),account_id.clone()));
			ensure!(_manger_operator_index.is_some(),"_manger_operator_index: manager is not  the operator's owner");

			let old_operator_index = Self::next_operator_index()?;
			let new_operator_index = old_operator_index - 1.into();
			<OperatorCount<T>>::put(new_operator_index);

			// let Some(_remove_operator_index) = _manger_operator_index;
			<Operators<T>>::remove(_manger_operator_index.unwrap());
			//TODO <ManagerOwners<T>>::insert(new_manage_index, account_id.clone());
			<OperatorToIndex<T>>::remove(account_id.clone());
			<CreatorMangerList<T>>::remove((sender.clone(),account_id.clone()));

			// remove role to the PostManageList
			<ManagerOperatorList<T>>::remove((sender.clone(),account_id.clone()));
			
			// remove event
			Self::deposit_event(RawEvent::RemoveRoleList(sender, account_id));

			Ok(())
		}		
	}
}
impl<T: Trait> Module<T> {
	
	///get the next manager index
	fn next_manager_index() -> result::Result<T::RoleIndex, &'static str> {
		let manager_index = Self::manager_count();
		if manager_index == T::RoleIndex::max_value() {
			return Err("manager count overflow");
		}
		Ok(manager_index)
	}

	///get the next operator index
	fn next_operator_index() -> result::Result<T::RoleIndex, &'static str> {
		let operator_inex = Self::operator_count();
		if operator_inex == T::RoleIndex::max_value() {
			return Err("operator count overflow");
		}
		Ok(operator_inex)
	}
}

decl_event!(
	pub enum Event<T> where 		
		<T as system::Trait>::AccountId,
		<T as Trait>::RoleIndex,
		{

		//add the Account to the post role list include Manage,operator
		AddRoleList(AccountId, AccountId, RoleIndex),

		// remove the role of the Account
		RemoveRoleList(AccountId, AccountId),

	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, parameter_types};
	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	}
	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			// assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
