#


3

apt update && apt install net-tools
IP=$(ifconfig eth0 | sed -En 's/.*inet (addr:)?([0-9.]+).*/\2/p' | head -n1)
mkdir -p /klaytn
echo "[\"kni://27ad162be6c22648d4e667c5e580a3b4b071bb0f2ad62064ce4d3702dbfbc41b5d953d88db14720ca55c6e31ca23deb1a97986f4eb4da717db5636e8951e1e04@$IP:32323?discport=0\u0026ntype=cn\"]" > /klaytn/static-nodes.json
kcn --datadir "/klaytn" init "/klaytn/genesis.json"
echo "# docker-compose" >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'NETWORK=""' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'DATA_DIR="/klaytn"' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'LOG_DIR="$$DATA_DIR/log"' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'RPC_ENABLE=1' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'RPC_API="db,eth,klay,net,governance,web3,miner,personal,txpool,debug,admin,istanbul,mainbridge,subbridge"' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'NETWORK_ID="2018"' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'NO_DISCOVER=1' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'ADDITIONAL="$$ADDITIONAL --identity \"CN-0\""' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'ADDITIONAL="$$ADDITIONAL --nodekeyhex 9db4f4ff652ef637583536e6b7b9bc1f5188af6842d498bf1adb5caeb9e64356"' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'REWARDBASE=0xfd99C5960b0024e5c5C48b27963088bbC33c9b94' >> /klaytn-docker-pkg/conf/kcnd.conf
echo 'ADDITIONAL="$$ADDITIONAL --debug --metrics --prometheus"' >> /klaytn-docker-pkg/conf/kcnd.conf
kcn --rewardbase 0xfd99C5960b0024e5c5C48b27963088bbC33c9b94 --rpc --rpcapi db,eth,klay,net,governance,web3,miner,personal,txpool,debug,admin,istanbul,mainbridge,subbridge --nodiscover --discover-types cn
# /klaytn-docker-pkg/bin/kcnd start
# sleep 1
# ken attach --exec "personal.importRawKey('9db4f4ff652ef637583536e6b7b9bc1f5188af6842d498bf1adb5caeb9e64356', '')" http://localhost:8551
# ken attach --exec "personal.unlockAccount('0xfd99C5960b0024e5c5C48b27963088bbC33c9b94', '', 999999999)" http://localhost:8551
# tail -F klaytn/log/kcnd.out
