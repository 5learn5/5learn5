#.\reCreate.ps1

near call lamp.its.testnet new_lamp '{\"ltype\": \"Neon\", \"state\": \"Off\"}' --accountId lamp.its.testnet
near call lamp.its.testnet new_lamp '{\"ltype\": \"Halogen\", \"state\": \"On\"}' --accountId lamp.its.testnet
near call lamp.its.testnet new_lamp '{\"ltype\": \"Neon\", \"state\": \"On\"}' --accountId lamp.its.testnet
near call lamp.its.testnet new_lamp '{\"ltype\": \"Halogen\", \"state\": \"On\"}' --accountId lamp.its.testnet
near view lamp.its.testnet get_all_lamps

