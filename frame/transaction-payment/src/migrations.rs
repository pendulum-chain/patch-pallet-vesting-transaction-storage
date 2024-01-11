
use frame_support::traits::OnRuntimeUpgrade;
use super::*;
// temporary code to set the migration to V1
pub mod v1 {
	use super::*;

	/// Manually setting the version to V1
	pub struct ForceSetVersionToV2<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for ForceSetVersionToV2<T> {
		fn on_runtime_upgrade() -> Weight {

			if StorageVersion::<T>::get() == Releases::V1Ancient {
				StorageVersion::<T>::put(Releases::V2);
			}
			
			T::DbWeight::get().reads_writes(1, 1)
		}
	
	}

}