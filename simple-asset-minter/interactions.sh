WASM_PATH='output/simple-asset-minter.wasm'
WALLET='~/Desktop/devKey.pem'
PROXY='https://gateway.multiversx.com'
CHAIN_ID='1'
GAS_LIMIT=60000000

deploy() {
	mxpy --verbose contract deploy --bytecode="$WASM_PATH" --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
		--outfile="deploy.interaction.json" \
		--gas-limit=${GAS_LIMIT} \
		--metadata-payable-by-sc

	local address=$(mxpy data parse --file="deploy.interaction.json" --expression="data['contractAddress']")
	echo "  Address: $address"

	# Store the address using mxpy
	mxpy data store --key=address-env-${CHAIN_ID} --value=${address}

}

issue_semi_fungible() {
	local token_name="$1"
	local token_ticker="$2"
	local address=$(get_stored_address "address-env-${CHAIN_ID}")

	echo "Issuing token $token_name ($token_ticker) using $address"

	local arg1=$(str2hex "$token_name")
	local arg2=$(str2hex "$token_ticker")

	mxpy --verbose contract call "$address" \
		--pem=$WALLET --recall-nonce \
		--proxy=$PROXY --chain=$CHAIN_ID \
		--gas-limit=60000000 --send \
		--function="issueSemiFungible" --value=50000000000000000 \
		--arguments "0x$arg1" "0x$arg2"
}

main() {
	deploy
	sleep 12
	issue_semi_fungible "WeaponTokens" "WTOKENS"
	sleep 12
	issue_semi_fungible "ArmorTokens" "ATOKENS"
	sleep 12
	issue_semi_fungible "LandTokens" "LANDTOKENS"
	sleep 12
	issue_semi_fungible "LandChests" "LANDCHEST"
	sleep 12
	issue_semi_fungible "MiscTokebs" "MISCTOKENS"
	sleep 12
	issue_semi_fungible "CollectibleTokens" "CTOKENS"
}

#######################################
# 			Utility functions		  #
#######################################
get_stored_address() {
	local key=$1
	local data_storage_file="mxpy.data-storage.json"

	if [ ! -f "$data_storage_file" ]; then
		echo "Error: Data storage file $data_storage_file not found." >&2
		return 1
	fi

	local address=$(grep "\"$key\"" "$data_storage_file" | sed -E 's/.*"'"$key"'": "([^"]+)".*/\1/')

	if [ -z "$address" ]; then
		echo "Error: Address for key $key not found in data storage." >&2
		return 1
	fi

	echo "$address"
}

str2hex() {
	local input="$1"
	# Use printf to ensure no additional characters are added
	local hex=$(printf '%s' "$input" | xxd -p -c 200 | tr -d '\n')
	echo "$hex"
}

hexStr2str() {
	local hexStr="$1"
	local str=$(echo "$hexStr" | xxd -r -p)
	echo "$str"
}

# Function to convert integer to hex with even length
int2hex() {
	local input="$1"
	local hex=$(printf "%x" "$input")

	# Ensure even length
	if [ $((${#hex} % 2)) -ne 0 ]; then
		hex="0$hex"
	fi

	echo "$hex"
}

addr2hex() {
	local input="$1"

	local hex=$(mxpy wallet bech32 --decode $input)

	echo "$hex"
}
