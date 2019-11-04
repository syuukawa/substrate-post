/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, 
			StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;

///add the mod 
use crate::post_parcel::Trait as PP_Trait;
use crate::post_parcel::{ParcelStruct, Parcels,ParcelReceiver};

use crate::post_role::Trait as PR_Trait;

/// The module's configuration trait.
pub trait Trait: system::Trait + PP_Trait + PR_Trait{
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Receiver {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		// Something get(something): Option<u32>;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		///update_coordinates_state: parcel_index,_coordinates,_state
		///origin: the operator account
		pub fn update_parcel_state_to_received(origin, _parcel_index: T::ParcelIndex, _state_params: u32) -> Result {

			/// check it was signed.
			let _receiver = ensure_signed(origin)?;
			
			let _parcel_receiver = <ParcelReceiver<T>>::get(_parcel_index);
			ensure!(_parcel_receiver.unwrap() == _receiver, "The account is not the receiver, so can not update the state");

			let _parcel_option = <Parcels<T>>::get(_parcel_index);
			ensure!(_parcel_option.is_some(), "can not get the parcel by parcel_index");
			
			let _parcel_data = _parcel_option.unwrap();
			// let _parcel_receiver = _parcel_data.receiver;

			/// reset the data
			let parcel_data  = ParcelStruct {
				p_id: _parcel_index,
				p_source: _parcel_data.p_source,
				p_destination: _parcel_data.p_destination,
				p_receiver: _parcel_data.p_receiver,
				p_coordinates: _parcel_data.p_coordinates,
				p_state: _state_params,
			};
			
			///先删除，再添加
			<Parcels<T>>::remove(_parcel_index);
			<Parcels<T>>::insert(_parcel_index, parcel_data);

			/// event
			Self::deposit_event(RawEvent::UpdateParcelStateToReceived(_receiver, _parcel_index, _state_params));

			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where <T as system::Trait>::AccountId,
		<T as PP_Trait>::ParcelIndex {
		/// Just a  Update Parcel State To Received event.
		UpdateParcelStateToReceived(AccountId, ParcelIndex, u32),
	}
);

// /// tests for this module
// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	use runtime_io::with_externalities;
// 	use primitives::{H256, Blake2Hasher};
// 	use support::{impl_outer_origin, assert_ok, parameter_types};
// 	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
// 	use sr_primitives::weights::Weight;
// 	use sr_primitives::Perbill;

// 	impl_outer_origin! {
// 		pub enum Origin for Test {}
// 	}

// 	// For testing the module, we construct most of a mock runtime. This means
// 	// first constructing a configuration type (`Test`) which `impl`s each of the
// 	// configuration traits of modules we want to use.
// 	#[derive(Clone, Eq, PartialEq)]
// 	pub struct Test;
// 	parameter_types! {
// 		pub const BlockHashCount: u64 = 250;
// 		pub const MaximumBlockWeight: Weight = 1024;
// 		pub const MaximumBlockLength: u32 = 2 * 1024;
// 		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
// 	}
// 	impl system::Trait for Test {
// 		type Origin = Origin;
// 		type Call = ();
// 		type Index = u64;
// 		type BlockNumber = u64;
// 		type Hash = H256;
// 		type Hashing = BlakeTwo256;
// 		type AccountId = u64;
// 		type Lookup = IdentityLookup<Self::AccountId>;
// 		type Header = Header;
// 		type WeightMultiplierUpdate = ();
// 		type Event = ();
// 		type BlockHashCount = BlockHashCount;
// 		type MaximumBlockWeight = MaximumBlockWeight;
// 		type MaximumBlockLength = MaximumBlockLength;
// 		type AvailableBlockRatio = AvailableBlockRatio;
// 		type Version = ();
// 	}
// 	impl Trait for Test {
// 		type Event = ();
// 	}
// 	type TemplateModule = Module<Test>;

// 	// This function basically just builds a genesis storage key/value store according to
// 	// our desired mockup.
// 	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
// 		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// 	}

// 	#[test]
// 	fn it_works_for_default_value() {
// 		with_externalities(&mut new_test_ext(), || {
// 			// Just a dummy test for the dummy funtion `do_something`
// 			// calling the `do_something` function with a value 42
// 			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 			// asserting that the stored value is equal to what we stored
// 			assert_eq!(TemplateModule::something(), Some(42));
// 		});
// 	}
// }
