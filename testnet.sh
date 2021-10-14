#!/bin/bash
# Global Variables
yellow=`tput setaf 3`
green=`tput setaf 4`
purple=`tput setaf 5`
red=`tput setaf 1`
reset=`tput sgr0`
white_bg=`tput setab 0`
red_bg=`tput setab 1`
NODE_ADDRESS=http://135.181.162.15:7777
DEPLOY_AMOUNT=200000000000
QUERY_AMOUNT=1921300000
CSPR_HOLDER_HASH=hash-9d6e519c1c0981c178370e283369278ec0e594ab05c64a7648cc104fcf344c2f
#The following hash was a result of deploying the ERC20 token from its contract.
#ERC20_HASH=hash-3f38843e639f144dc1535999becfdad347b8518736b9dfd9a0430fd60e366d5f
#The following hash was a result of deploying the ERC20 token from from the factory contract.
ERC20_HASH=hash-46f58aaf8fff34d5354c1453e645c3ea08fb43908f3468a1b778bb6f4fb946be
FACTORY_HASH=hash-4bc5f6d0cfdcb2c8ceb54abfbf370a6f3c229ab89d101a9109ca9a43f457428d
ERC20_SESSION_PATH=./target/wasm32-unknown-unknown/release/erc20.wasm
CSPR_HOLDER_SESSION_PATH=./target/wasm32-unknown-unknown/release/cspr_holder.wasm
FACTORY_SESSION_PATH=./target/wasm32-unknown-unknown/release/factory.wasm
GOVERNANCE_KEY=./keys/governance/secret_key.pem
ERC20_KEY=./keys/erc20/secret_key.pem
CSPR_HOLDER_KEY=./keys/cspr_holder/secret_key.pem
FACTORY_KEY=./keys/factory/secret_key.pem
if [[ $1 == 'erc20' ]]
then
  if [[ $2 == 'deploy' ]]
  then
    if [[ $3 == '' ]]
    then
      echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
      echo "[✔] ${red}erc20 ${purple}deploy${reset} <GOVERNANCE>"
      exit 0
    fi
    pushd ./erc20
    cargo build --release
    popd
    casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${ERC20_SESSION_PATH} --secret-key ${ERC20_KEY} --session-arg "token_name:string='ERC20'" "token_symbol:string='ERC'" "token_decimals:u8='8'" "token_total_supply:u256='1000'" "governance:account_hash='$3'"
  elif [[ $2 == 'query' ]]
  then
    if [[ $3 == '' ]]
    then
      echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
      echo "[✔] ${red}erc20 ${purple}query${reset} <ENDPOINT>"
      echo "[i] ${yellow}<ENDPOINT> ∈ {transfer, transfer_from, approve, mint, burn}${reset}"
      exit 0
    fi
    if [[ $3 == 'approve' ]]
    then
      if [[ $4 == '' || $5 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} ${green}approve${reset} <SPENDER> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${GOVERNANCE_KEY} --session-hash ${ERC20_HASH} --session-entry-point approve --session-arg "spender:key='$4'" "amount:u256='$5'"
      fi
    elif [[ $3 == 'transfer' ]]
    then
      if [[ $4 == '' || $5 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} ${green}transfer${reset} <RECIPIENT> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${GOVERNANCE_KEY} --session-hash ${ERC20_HASH} --session-entry-point transfer --session-arg "recipient:key='$4'" "amount:u256='$5'"
      fi
    elif [[ $3 == 'transfer_from' ]]
    then
      if [[ $4 == '' || $5 == '' || $6 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} ${green}transfer_from${reset} <OWNER> <RECIPIENT> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${FACTORY_KEY} --session-hash ${ERC20_HASH} --session-entry-point transfer_from --session-arg "owner:key='$4'" "recipient:key='$5'" "amount:u256='$6'"
      fi
    elif [[ $3 == 'mint' ]]
    then
      if [[ $4 == '' || $5 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} ${green}mint${reset} <OWNER> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${GOVERNANCE_KEY} --session-hash ${ERC20_HASH} --session-entry-point mint --session-arg "owner:key='$4'" "amount:u256='$5'"
      fi
    elif [[ $3 == 'burn' ]]
    then
      if [[ $4 == '' || $5 == '' ]]
      then
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}erc20 ${purple}query${reset} ${green}burn${reset} <OWNER> <AMOUNT>"
        exit 0
      else
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${GOVERNANCE_KEY} --session-hash ${ERC20_HASH} --session-entry-point burn --session-arg "owner:key='$4'" "amount:u256='$5'"
      fi
    fi 
  fi
elif [[ $1 == 'cspr_holder' ]]
then
  if [[ $2 == 'deploy' ]]
  then
    if [[ $3 != '' ]]
    then
        pushd ./cspr-holder
        cargo build --release
        popd
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${CSPR_HOLDER_SESSION_PATH} --secret-key ${CSPR_HOLDER_KEY} --session-arg "governance:account_hash='$3'"
    else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}cspr_holder ${purple}deploy${reset} <GOVERNANCE>"
        exit 0
    fi
  elif [[ $2 == 'query' ]]
  then
    #   ====Getters (endpoints that return data using ret()) will generate a Return Error when being queried on testnet.====
    #   ====They were tested locally and since the tests on testnet went smoothly, they should work perfectly.====
    if [[ $3 == 'lock' ]]
    then
      if [[ $4 != '' && $5 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${ERC20_KEY} --session-hash ${CSPR_HOLDER_HASH} --session-entry-point lock --session-arg "src_purse:uref='$4'" "amount:u512='$5'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}cspr_holder ${purple}query ${green}lock${reset} <SRC_PURSE> <AMOUNT>"
        exit 0
      fi
    elif [[ $3 == 'unlock' ]]
    then
      if [[ $4 != '' && $5 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${QUERY_AMOUNT} --secret-key ${GOVERNANCE_KEY} --session-hash ${CSPR_HOLDER_HASH} --session-entry-point unlock --session-arg "target_pubkey:public_key='$4'" "amount:u512='$5'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}cspr_holder ${purple}query ${green}unlock${reset} <TARGET_PUBKEY> <AMOUNT>"
        exit 0
      fi
    fi
  fi
elif [[ $1 == 'factory' ]]
then
  if [[ $2 == 'deploy' ]]
  then
    if [[ $3 != '' ]]
    then
        pushd ./factory
        cargo build --release
        popd
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --session-path ${FACTORY_SESSION_PATH} --secret-key ${FACTORY_KEY} --session-arg "governance:account_hash='$3'"
    else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}factory ${purple}deploy${reset} <GOVERNANCE>"
        exit 0
    fi
  elif [[ $2 == 'query' ]]
  then
    if [[ $3 == 'create_erc20' ]]
    then
      if [[ $4 != '' && $5 != '' && $6 != '' && $7 != '' && $8 != '' ]]
      then
        casper-client put-deploy --chain-name casper-test --node-address ${NODE_ADDRESS} --payment-amount ${DEPLOY_AMOUNT} --secret-key ${GOVERNANCE_KEY} --session-hash ${FACTORY_HASH} --session-entry-point create_erc20 --session-arg "token_name:string='$4'" "token_symbol:string='$5'" "token_decimals:u8='$6'" "token_total_supply:u256='$7'" "governance:account_hash='$8'"
      else
        echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
        echo "[✔] ${red}factory ${purple}query ${green}create_erc20${reset} <NAME> <SYMBOL> <DECIMALS> <TOTAL_SUPPLY> <GOVERNANCE>"
        exit 0
      fi
    fi
  fi
fi
if [[ $1 == 'check_status' ]]
then
  if [[ $2 == '' ]]
  then
    echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
    echo "[✔] ${red}check_status${reset} <DEPLOY_HASH>"
    exit 0
  fi
  casper-client get-deploy --node-address ${NODE_ADDRESS} $2 > deploy_status.json
elif [[ $1 == 'get_state_root_hash' ]]
then
  casper-client get-state-root-hash --node-address ${NODE_ADDRESS} | jq -r
elif [[ $1 == 'query_state' ]]
then
  if [[ $2 == '' || $3 == '' ]]
  then
    echo "${red_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ Invalid Syntax! ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
    echo "[✔] ${red}query_state${reset} <DEPLOYER_PUBKEY> <STATE_ROOT_HASH>"
    exit 0
  fi
  casper-client query-state --node-address ${NODE_ADDRESS} -k $2 -s $3 |jq -r
elif [[ $1 == 'examples' ]]
then 
  casper-client put-deploy --show-arg-examples
elif [[ $1 == 'syntax' ]]
then
  echo "${yellow}${white_bg}↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴ 𝐒𝐘𝐍𝐓𝐀𝐗 ツ ↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴↴${reset}"
  echo "[✔] ${red}examples${reset}"
  echo "[✔] ${red}check_status${reset} <DEPLOY_HASH>"
  echo "[✔] ${red}get_state_root_hash${reset}"
  echo "[✔] ${red}query_state${reset} <DEPLOYER_PUBKEY> <STATE_ROOT_HASH>"
  echo "[✔] ${red}erc20 ${purple}deploy${reset} <GOVERNANCE>"
  echo "[✔] ${red}erc20 ${purple}query${reset} ${green}approve${reset} <SPENDER> <AMOUNT>"
  echo "[✔] ${red}erc20 ${purple}query${reset} ${green}transfer${reset} <RECIPIENT> <AMOUNT>"
  echo "[✔] ${red}erc20 ${purple}query${reset} ${green}transfer_from${reset} <OWNER> <RECIPIENT> <AMOUNT>"
  echo "[✔] ${red}erc20 ${purple}query${reset} ${green}mint${reset} <OWNER> <AMOUNT>"
  echo "[✔] ${red}erc20 ${purple}query${reset} ${green}burn${reset} <OWNER> <AMOUNT>"
  echo "[✔] ${red}cspr_holder ${purple}deploy${reset} <GOVERNANCE>"
  echo "[✔] ${red}cspr_holder ${purple}query ${green}lock${reset} <SOURCE_PURSE> <AMOUNT>"
  echo "[✔] ${red}cspr_holder ${purple}query ${green}unlock${reset} <TARGET_PUBKEY> <AMOUNT>"
  echo "[✔] ${red}factory ${purple}deploy${reset} <GOVERNANCE>"
  echo "[✔] ${red}factory ${purple}query ${green}create_erc20${reset} <NAME> <SYMBOL> <DECIMALS> <TOTAL_SUPPLY> <GOVERNANCE>"
fi