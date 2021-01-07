use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_runtime::{
    ModuleId,
    traits::{BlakeTwo256, IdentityLookup},
    testing::Header, Perbill,
};
use orml_currencies::BasicCurrencyAdapter;

pub type AccountId = u128;
pub type Amount = i128;
pub type Balance = u128;
pub type CurrencyId = u128;

pub const ALICE: AccountId = 0;
pub const BOB: AccountId = 1;

impl_outer_origin! {
    pub enum Origin for Test {}
}
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl orml_tokens::Trait for Test {
    type Event = ();
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
	type OnReceived = ();
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 10;
	pub const MaxLocks: u32 = 10;
}

impl pallet_balances::Trait for Test {
	type Balance = u128;
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<Test>;
	type WeightInfo = ();
	type MaxLocks = MaxLocks;
}

parameter_types! {
	pub const GetNativeCurrencyId: u128 = 0;
}

impl orml_currencies::Trait for Test {
	type Event = ();
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Test, Balances, i128, u32>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}

parameter_types! {
    pub const CurveDeposit: Balance = 10;
    pub const BondingCurveModuleId: ModuleId = ModuleId(*b"mtg/bonc");
}

impl Trait for Test {
    type Event = ();
    type Currency = Currencies;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type CurveDeposit = CurveDeposit;
    type ModuleId = BondingCurveModuleId;
}

pub type Balances = pallet_balances::Module<Test>;
pub type Currencies = orml_currencies::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Tokens = orml_tokens::Module<Test>;
pub type BondingCurve = Module<Test>;

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
	balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, 0, 1_000),
				(BOB, 0, 1_000),

			],
			balances: vec![
				(ALICE, 1_000_000_000_000),
				(BOB, 1_000_000_000_000),
			]
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();
		
		orml_tokens::GenesisConfig::<Test> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: self.balances,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
