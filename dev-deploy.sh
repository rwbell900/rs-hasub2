#!/usr/bin/env bash

[ -z "$CONTRACT" ] && echo "Missing \$CONTRACT environment variable"
[ -z "$OWNER" ] && echo "Missing \$OWNER environment variable"

echo "deleting $CONTRACT and setting $OWNER as beneficiary"
echo
near delete $CONTRACT $OWNER

echo --------------------------------------------
echo
echo "cleaning up the /neardev folder"
echo
rm -rf ./neardev

# exit on first error after this point to avoid redeploying with successful build
set -e

echo --------------------------------------------
echo
echo "rebuilding the contract (release build)"
echo
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/*.wasm ./res/

echo --------------------------------------------
echo
echo "redeploying the contract"
echo
near dev-deploy ./res/rs_hasub2.wasm

echo --------------------------------------------
echo run the following commands
echo

echo "export CONTRACT=<dev-1653168890126-82647320324793>"
echo "export OWNER=your account Id"
echo "near call \$CONTRACT new '{}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_name '{\"name\":\"Alices non-profit\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_status '{\"status\":\"Applicant\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_description '{\"description\":\"We make healthier food available to more people\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_status '{\"status\":\"Applicant\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_allowed_inventory_bags '{\"allowed_inventory_bags\":40}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_allowed_inventory_cases '{\"allowed_inventory_cases\":20}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_bag_inventory '{\"bag_inventory\":25}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_case_inventory '{\"bag_inventory\":10}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_status '{\"status\":\"Applicant\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_owner '{\"owner\":\"'\$OWNER'\"}' --accountId \$OWNER"
echo "near call \$CONTRACT check_owner '{\"owner\":\"'\$OWNER'\"}' --accountId \$OWNER"
echo "near call \$CONTRACT check_owner '{\"owner\":\"badaccountname.testnet\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_status '{\"status\":\"Approved\"}' --accountId \$OWNER"
echo "near call \$CONTRACT set_entity_status '{\"status\":\"Badstatus\"}' --accountId \$OWNER"
echo
echo
exit 0