
near view lamp.trying.testnet get_all_lamps
#pause

near view lamp.trying.testnet can_set_lamp_state  '{\"new_state\": \"On\"}' 
near view lamp.trying.testnet can_set_lamp_state  '{\"new_state\": \"Off\"}'
#pause 

near call lamp.trying.testnet set_lamp_state '{\"lamp_id\": 1, \"new_state\": \"On\"}' --accountId trying.testnet
near call lamp.trying.testnet set_lamp_state '{\"lamp_id\": 2, \"new_state\": \"Off\"}' --accountId trying.testnet
#pause

near call lamp.trying.testnet disable_lamp '{\"lamp_id\": 1}' --accountId lamp.trying.testnet
near view  lamp.trying.testnet get_lamp '{\"lamp_id\": 1}'
#pause
near call lamp.trying.testnet enable_lamp '{\"lamp_id\": 1}' --accountId lamp.trying.testnet
near view  lamp.trying.testnet get_lamp '{\"lamp_id\": 1}'
#pause

near call lamp.trying.testnet calculate_reward '{\"reward_for\": \"Self\"}'  --accountId trying.testnet
near call lamp.trying.testnet calculate_reward '{\"reward_for\": \"All\"}'  --accountId lamp.trying.testnet
#pause

near view lamp.trying.testnet get_owner
#pause
near call lamp.trying.testnet change_owner '{\"new_owner\": \"trying.testnet\"}' --accountId lamp.trying.testnet
near view lamp.trying.testnet get_owner
