#!/usr/bin/env bash

price=$(curl -s -H "X-CMC_PRO_API_KEY: ${CMC_API_KEY}" -H "Accept: application/json" \
    -d "id=1&convert=USD" -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest \
    | jq '.data["1"].quote.USD.price')

near call "$CONTRACT_ACCOUNT" add_price "{\"price\": $price}" --accountId "$USER_ACCOUNT"
