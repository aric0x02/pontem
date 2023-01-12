# Clone move-stdlib
rm -rf ./move-stdlib
git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
pushd ./move-stdlib
git checkout release-v1.0.0
dove build
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
pushd ./pont-stdlib
git checkout release-v1.0.0
dove build
popd

pushd ./user
dove clean
dove build
dove call "store_u64(42)"
dove call "emit_event(42)"
dove call "store_system_block()"
dove call "store_system_timestamp()"
dove call "inf_loop()"
dove call "store_native_balance()"
dove call "store_token_balance()"
dove call "as_root(dr)"
dove call "transfer<0x1::NOX::NOX>(Alice, 2000)" -o=transfer.mvt
dove call "transfer<0x1::KSM::KSM>(Alice, 2000)" -o=transfer_token.mvt
dove call "multisig_test()"
dove call "deposit_bank<0x1::NOX::NOX>(2000)" -o=deposit_bank_pont.mvt
dove call "deposit_bank<0x1::KSM::KSM>(2000)" -o=deposit_bank_ksm.mvt
dove call "signer_one()" -o=signer_user.mvt
dove call "signer_one(root)" -o=signer_root.mvt
popd

pushd ./root
dove clean
dove build
pushd
