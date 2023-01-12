# Clone move-stdlib
rm -rf ./move-stdlib
git clone https://github.com/pontem-network/move-stdlib ./move-stdlib
pushd ./move-stdlib
git checkout release-v1.0.0
dove build
dove deploy
popd

# Clone pont-stdlib
rm -rf ./pont-stdlib
git clone https://github.com/pontem-network/pont-stdlib.git ./pont-stdlib
pushd ./pont-stdlib
git checkout release-v1.0.0
dove build
dove deploy
popd

pushd ./user
dove clean
dove build
dove call "store_system_block()"
dove call "store_system_timestamp()"
dove call "transfer<0x1::NOX::NOX>(Alice, 500000000000)" -o=transfer_pont.mvt
dove call "transfer<0x1::KSM::KSM>(Alice, 500000000000)" -o=transfer_ksm.mvt
dove call "deposit_bank<0x1::NOX::NOX>(500000000000)" -o=deposit_bank_pont.mvt
dove call "deposit_bank<0x1::KSM::KSM>(50000000000000)" -o=deposit_bank_ksm.mvt
popd
