#!/bin/bash

docker-compose -f ./tests/docker-compose.yml build
docker-compose -f ./tests/docker-compose.yml up -d
HOST_2_LOOPBACK_IP=10.100.220.3
docker-compose -f ./tests/docker-compose.yml exec \
    -T host1 ping -c 5 $HOST_2_LOOPBACK_IP

# BGPでルート交換し，pingが通れば0, 通らなければ1がdocker-compose execの終了コード
TEST_RESULT=$?
if [$TEST_RESULT -eq 0]; then
    printf "\e[32m%s\e[m\n" "Success: integration test"
else 
    printf "\e[31m%s\e[m" "Failed: integration test"
fi 

exit $TEST_RESULT