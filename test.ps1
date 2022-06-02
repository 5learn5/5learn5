
near view lamp.its.testnet get_all_lamps
pause

near view lamp.its.testnet can_set_lamp_state  '{\"new_state\": \"On\"}' 
near view lamp.its.testnet can_set_lamp_state  '{\"new_state\": \"Off\"}'
pause 

near call lamp.its.testnet set_lamp_state '{\"lamp_id\": 1, \"new_state\": \"On\"}' --accountId its.testnet
near call lamp.its.testnet set_lamp_state '{\"lamp_id\": 2, \"new_state\": \"Off\"}' --accountId its.testnet
pause

near call lamp.its.testnet disable_lamp '{\"lamp_id\": 1}' --accountId lamp.its.testnet
near view  lamp.its.testnet get_lamp '{\"lamp_id\": 1}'
pause
near call lamp.its.testnet enable_lamp '{\"lamp_id\": 1}' --accountId lamp.its.testnet
near view  lamp.its.testnet get_lamp '{\"lamp_id\": 1}'
pause

near call lamp.its.testnet calculate_reward '{\"reward_for\": \"Self\"}'  --accountId its.testnet
near call lamp.its.testnet calculate_reward '{\"reward_for\": \"All\"}'  --accountId lamp.its.testnet
pause

near view lamp.its.testnet get_owner
pause
near call lamp.its.testnet change_owner '{\"new_owner\": \"new.its.testnet\"}' --accountId lamp.its.testnet
near view lamp.its.testnet get_owner
