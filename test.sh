# source ./neardev/dev-account.env
source ./neardev/dev-account.env

MESSAGE="Hello World"

npx near-cli call $CONTRACT_NAME set_status "{\"message\":\"$MESSAGE\"}" --accountId $CONTRACT_NAME

npx near-cli view $CONTRACT_NAME get_status --accountId $CONTRACT_NAME