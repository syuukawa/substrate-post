/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, 
			StorageValue, StorageMap, Parameter, dispatch::Result, ensure};
use sr_primitives::traits::{SimpleArithmetic, Bounded, Member};
// use runtime_io::blake2_128;
use system::ensure_signed;
use codec::{Encode, Decode};
use rstd::vec::Vec;
use rstd::result;

use crate::linked_item::{LinkedList, LinkedItem};
use crate::post_parcel::Trait as ParcelTrait;

use crate::post_role::Trait as PR_Trait;
use crate::post_role::{ OperatorToIndex };

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct ParcelStruct <T: system::Trait + ParcelTrait> {
	pub p_id: T::ParcelIndex,
	pub p_receiver: T::AccountId,
	pub p_state: u32,
	pub p_source: [u8; 16], 
	pub p_destination: [u8; 16],
	pub p_coordinates: Vec<(i16,i16)>,
}
///parcel_state未发货，已发货，已收获

type ParcelLinkedItem<T> = LinkedItem<<T as Trait>::ParcelIndex>;
type ParcelsList<T> = LinkedList<OwnedParcels<T>, <T as system::Trait>::AccountId, <T as Trait>::ParcelIndex>;

/// The module's configuration trait.
pub trait Trait: system::Trait + PR_Trait {

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	///the ParcelIndex
	type ParcelIndex: Parameter + Member + SimpleArithmetic + Bounded + Default + Copy;

}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as PostParcel {

		/// Stores the total number of parcel. i.e. the next kitty index
		pub ParcelCount get(parcel_count): T::ParcelIndex;

		/// Stores all the Parcel, key is the parcel id/index
		pub Parcels get(parcels): map T::ParcelIndex => Option<ParcelStruct<T>>;

		/// the operator owned the parcels
		pub OwnedParcels get(owned_parcels): map (T::AccountId, Option<T::ParcelIndex>) => Option<ParcelLinkedItem<T>>;

		/// Get Parcels owner
		pub ParcelOwners get(parcel_owners): map T::ParcelIndex => Option<T::AccountId>;

		/// Get Parcels owner
		pub ParcelReceiver get(parcel_receiver): map T::ParcelIndex => Option<T::AccountId>;

		/// creator and the manager list
		pub OperatorParcelList get(operator_parcel_list): map (T::AccountId, T::AccountId) => Option<T::ParcelIndex>;

	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		///origin: the operator account
		pub fn create_parcel(origin, _receiver: T::AccountId, _state_params: u32, _source: [u8; 16], _destination: [u8; 16], _coordinates: (i16,i16)) -> Result {

			/// check it was signed.
			let _operater = ensure_signed(origin)?;
			
			/// check the operator
			let _operator_index = <OperatorToIndex<T>>::get(_operater.clone());
			ensure!(_operator_index.unwrap() >  0.into() ,"the origin is not a operator");

			let old_parcel_index = Self::next_parcel_index()?;
			let new_parcel_index = old_parcel_index + 1.into();
			<ParcelCount<T>>::put(new_parcel_index);

			let mut _coordinates_params:Vec<(i16,i16)> = Vec::new();
			_coordinates_params.push(_coordinates);
			
			let _parcel_data = ParcelStruct {
				p_id: new_parcel_index,
				p_receiver: _receiver.clone(),
				p_state: _state_params,
				p_source: _source.clone(),
				p_destination: _destination.clone(),
				p_coordinates: _coordinates_params,
			};

			// Create and store Parcels
			<Parcels<T>>::insert(new_parcel_index, _parcel_data);
			<ParcelCount<T>>::put(new_parcel_index);
			<ParcelOwners<T>>::insert(new_parcel_index, _operater.clone());

			/// linkedItem
			<ParcelsList<T>>::append(&_operater, new_parcel_index);

			/// receiver
			<ParcelReceiver<T>>::insert(new_parcel_index,_receiver.clone());

			// TODO event
			// Self::deposit_event(RawEvent::CreateParcel(_operater, _source, _destination, _receiver, _coordinates_params));
			Ok(())
		}

		///_update_parcel_coordinates: parcel_index,_coordinates
		///origin: the receiver account
		pub fn _update_parcel_coordinates(origin, parcel_index: T::ParcelIndex, _coordinates_params: (i16,i16)) -> Result {

			/// heck it was signed.
			let _operater = ensure_signed(origin)?;

			//// check the percel operator by the operator and parcel_index
			ensure!(Self::parcel_owners(parcel_index).map(|owner| owner == _operater.clone()).unwrap_or(false), "Not the operator of Parcel");

			let post_parcel = Self::parcels(parcel_index);
			ensure!(post_parcel.is_some(), "can not  get the parcel by parcel_index");

			let post_parcel_data: ParcelStruct<T> = post_parcel.unwrap();

			let mut _coordinates: Vec<(i16, i16)> = post_parcel_data.p_coordinates;
			_coordinates.push(_coordinates_params);

			/// reset the data
			let parcel_data  = ParcelStruct {
				p_id: parcel_index,
				p_source: post_parcel_data.p_source,
				p_destination: post_parcel_data.p_destination,
				p_receiver: post_parcel_data.p_receiver,
				p_coordinates: _coordinates,
				p_state: post_parcel_data.p_state,
			};
			
			///先删除，再添加
			<Parcels<T>>::remove(parcel_index);
			<Parcels<T>>::insert(parcel_index, parcel_data);

			// TODO event
			// Self::deposit_event(RawEvent::CreateParcel(_operater, Vec<u32>, Vec<u32>, AccountId, (u32,u32), u8););
			Ok(())
		}

		// ///_receiver_update_parcel_state: parcel_index,_state
		// ///origin: the operator account
		// pub fn _receiver_update_parcel_state(origin, parcel_index: T::ParcelIndex, _state_params: u8) -> Result {

		// 	/// heck it was signed.
		// 	let _operater = ensure_signed(origin)?;

		// 	let _parcel_receiver = Self::parcel_receiver(parcel_index);
		// 	ensure!(_parcel_receiver.unwrap() == _operater, "The account is not the receiver, so can not update the state");

		// 	let post_parcel = Self::parcels(parcel_index);
		// 	ensure!(post_parcel.is_some(), "can not  get the parcel by parcel_index");

		// 	let post_parcel_data: Parcel<T> = post_parcel.unwrap();

		// 	/// reset the data
		// 	let parcel_data  = Parcel {
		// 		id: parcel_index,
		// 		parcel_state: _state_params,
		// 		source: post_parcel_data.source,
		// 		destination: post_parcel_data.destination,
		// 		receiver: post_parcel_data.receiver,
		// 		coordinates: post_parcel_data.coordinates,
		// 	};
			
		// 	///先删除，再添加
		// 	<Parcels<T>>::remove(parcel_index);
		// 	<Parcels<T>>::insert(parcel_index, parcel_data);

		// 	// TODO event
		// 	// Self::deposit_event(RawEvent::CreateParcel(_operater, Vec<u32>, Vec<u32>, AccountId, (u32,u32), u8););
		// 	Ok(())
		// }

	}
}

impl<T: Trait> Module<T> {

	///get the next parcel index
	fn next_parcel_index() -> result::Result<T::ParcelIndex, &'static str> {
		let parcel_index = Self::parcel_count();
		if parcel_index == T::ParcelIndex::max_value() {
			return Err("parcel count overflow");
		}
		Ok(parcel_index)
	}

	// /// get the random nonce
	// fn random_value(sender: &T::AccountId) -> [u8; 16] {
	// 	let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
	// 	payload.using_encoded(blake2_128)
	// }

	// // Generate a random 128bit value
	// let selector = Self::random_value(&sender);
}

decl_event!(
	pub enum Event<T> where 
		<T as system::Trait>::AccountId {
		// <T as Trait>::ParcelIndex
		// CreateParcel event
		CreateParcel(AccountId, u8,u8, AccountId, Vec<(i16,i16)>),
		// UpdateParcelCoordinates(AccountId, ParcelIndex, (i16,i16)),
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
			// // asserting that the stored value is equal to what we stored
			// assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
