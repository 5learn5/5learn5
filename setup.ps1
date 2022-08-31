#.\reCreate.ps1

near call lamp.trying.testnet new_lamp '{\"ltype\": \"Neon\", \"state\": \"Off\"}' --accountId lamp.trying.testnet
near call lamp.trying.testnet new_lamp '{\"ltype\": \"Halogen\", \"state\": \"On\"}' --accountId lamp.trying.testnet
near call lamp.trying.testnet new_lamp '{\"ltype\": \"Neon\", \"state\": \"On\"}' --accountId lamp.trying.testnet
near call lamp.trying.testnet new_lamp '{\"ltype\": \"Halogen\", \"state\": \"On\"}' --accountId lamp.trying.testnet
near view lamp.trying.testnet get_all_lamps

