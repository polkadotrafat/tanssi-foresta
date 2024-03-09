# Guide and List of Pallets to Add to Template Runtime

## STEPS TO ADD A PALLET TO RUNTIME

reference

```bash

https://docs.tanssi.network/builders/build/customize/adding-built-in-module/

```

### 1

- _Allow Dependencies_: Add listed package to main Cargo.toml in runtime as workspace = true and  main Cargo.toml with the github link to workspace to define the configuration settings and dependencies within:

```bash
runtime/Cargo.toml 

```

### 2

- _Allow Features_: Add the

```bash
pallet-name/std 
```

features to the list of features to enable when compiling the runtime - The features in each pallet that should be enabled when compiling the native Rust binary. By enabling the standard (std) feature set from each pallet, you can compile the runtime to include functions, types, and primitives that would otherwise be missing when you build the WebAssembly binary.

### 3

- _Check_: Check that the new dependencies resolve correctly by running the following command:

```bash
cargo check -p container-chain-template-simple-node --release

or

cargo build -p container-chain-template-simple-node --release
```

### 4

- _Add Config trait declaration_: Add Config traits to Runtime for pallet - Every pallet has a Rust trait called Config. The Config trait is used to identify the parameters and types that the pallet needs to carry out its functions.
 To review the Config trait for the Balances pallet:

```bash
#  Open the runtime/src/lib.rs file in a text editor.
#  Locate the impl pallet and note that it consists of the following implementation (impl)code block example:

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    /// The empty value, (), is used to specify a no-op callback function.
    type DustRemoval = ();
    /// Set the minimum balanced required for an account to exist on-chain
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    /// The FRAME runtime system is used to track the accounts that hold balances.
    type AccountStore = System;
    /// Weight information is supplied to the Balances pallet by the node template runtime.
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}
```

The Config must be implemented within the runtime/src/lib.rs file and then
/_**Add Listed Pallet Line within construct_runtime! macro**_/

### 5

- _Configure the Module in the Chain Specification_: Finally, add the configuration in the chain specification for the genesis state in the file chain_spec.rs located at:

```bash
container-chains/templates/frontier/node/src/chain_spec.rs
```

### 6

- _Check Dependency Resolve_: Check that the new dependencies resolve correctly by running the following command:

```bash
 cargo build -p container-chain-template-simple-node --release
```

### 7

- _Compile the Node_: Compile the node in release mode by running the following command and test the pallets extrinsics or events in the substrate node template:

```bash
 ./target/release/container-chain-template-simple-node --dev --sealing 6000
```

### 8

***Optional: only for packages that do not contain github linked code to substrate***

- _Add path code for local dependencies_: Add local dependencies code to folder to be accessible to Dependencies and Features**

## LIST OF PALLETS TO ADD

### Substrate

1. pallet-treasury = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }
2. pallet-uniques = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }
3. pallet-membership = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }
4. pallet-multisig = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }
5. pallet-scheduler = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }
6. pallet-preimage = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }
7. pallet-identity = { git = "<https://github.com/paritytech/substrate>", default-features = false, branch = "polkadot-v0.9.33" }

### ORML dependencies

1. orml-authority = { git = "<https://github.com/open-web3-stack/open-runtime-module-library>", default-features = false, branch = "polkadot-v0.9.33" }

2. orml-benchmarking = { git = "<https://github.com/open-web3-stack/open-runtime-module-library>", default-features = false, branch = "polkadot-v0.9.33" }

3. orml-nft = { git = "<https://github.com/open-web3-stack/open-runtime-module-library>", default-features = false, branch = "polkadot-v0.9.33" }

4. orml-tokens = { git = "<https://github.com/open-web3-stack/open-runtime-module-library>", default-features = false, branch = "polkadot-v0.9.33" }

5. orml-traits = { git = "<https://github.com/open-web3-stack/open-runtime-module-library>", default-features = false, branch = "polkadot-v0.9.33" }

### Local Dependencies

1. pallet-assets = { default-features = false, path = "../../pallets/assets" } :check

2. pallet-carbon-credits = { default-features = false, version = '0.0.1', path = "../../pallets/carbon-credits" } :check

3. pallet-carbon-credits-pool = { default-features = false, version = '0.0.1', path = "../../pallets/carbon-credits-pool" } :check

4. pallet-parachain-staking = { default-features = false, version = '0.0.1', path = "../../pallets/parachain-staking" }

5. pallet-transaction-pause = { default-features = false, version = '0.0.1', path = "../../pallets/transaction-pause" }

6. pallet-vesting-contract = { default-features = false, version = '0.0.1', path = "../../pallets/vesting-contract" }

7. pallet-kyc = { default-features = false, version = '0.0.1', path = "../../pallets/kyc" } :check

8. pallet-dex = { default-features = false, path = "../../pallets/dex" }

9. primitives = { package = "bitgreen-primitives", path = "../../primitives", default-features = false } :check
