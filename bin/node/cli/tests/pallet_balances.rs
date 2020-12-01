mod runtime;

use runtime::NodeTemplateChainInfo;
use substrate_test_runner::Node;
use pallet_sudo::Call as SudoCall;
use pallet_balances::Call as BalancesCall;
use sp_keyring::sr25519::Keyring::{Alice, Bob};
use sp_runtime::{traits::IdentifyAccount, MultiSigner};
use node_runtime::Call;

#[test]
fn test_force_transfer() {
    type Balances = pallet_balances::Module<node_runtime::Runtime>;
    let node = Node::<NodeTemplateChainInfo>::new().unwrap();
    let (alice, bob) = (
        MultiSigner::from(Alice.public()).into_account(),
        MultiSigner::from(Bob.public()).into_account(),
    );
    let (alice_balance, bob_balance) = node.with_state(|| (
        Balances::free_balance(alice.clone()),
        Balances::free_balance(bob.clone()),
    ));

    let balances_call = BalancesCall::force_transfer(alice.clone().into(), bob.clone().into(), alice_balance / 2);
    node.submit_extrinsic(
        SudoCall::sudo(Box::new(Call::Balances(balances_call))),
        alice
    );
    node.seal_blocks(1);

    let new_bob_balance = node.with_state(|| Balances::free_balance(bob.clone()));

    assert_eq!(new_bob_balance, bob_balance + (alice_balance / 2))
}