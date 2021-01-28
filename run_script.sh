#!/bin/sh

binary="./target/debug/framenode"

chain="local"

getopt_code=`awk -f ./misc/getopt.awk <<EOF
Usage: sh ./run_script.sh [OPTIONS]...
Run frame node based local test net
  -h, --help                         Show usage message
usage
exit 0
  -d, --duplicate-log-of-first-node  Duplicate log of first node to console
duplicate_log=1
  -w, --disable-offchain-workers     Disable offchain workers
offchain_flags="--offchain-worker Never"
  -r, --use-release-build            Use release build
binary="./target/release/framenode"
  -s, --staging                      Using staging chain spec
chain="staging"
EOF
`
eval "$getopt_code"


export RUST_LOG="eth_bridge=debug"

localid=`mktemp`
tmpdir=`dirname $localid`

if which gawk > /dev/null 2>&1; then
	awk="gawk"
else
	awk="awk"
fi

if [ ! -f $binary ]; then
	echo "Please build framenode binary"
	echo "for example by running command: cargo build --debug or cargo build --release"
	exit 1
fi

function local_id() {
  $awk "
    BEGIN { a=1 }
    /Local node identity is: /{
      if (a) {
        print \$10 > \"$localid\";
        fflush();
        a=0
      }
    }
    { print \"LOG: \" \$0; fflush() }
  "
}

function logger_for_first_node() {
	if [ "$duplicate_log" == "1" ]; then
		tee $1
	else
		cat > $1
	fi
}

find . -name "db*" -type d -maxdepth 1 -exec rm -rf {}/chains/sora-substrate-local/network {}/chains/sora-substrate-local/db \;

port="10000"
wsport="9944"
num="0"
for name in alice bob charlie dave eve
do
	newport=`expr $port + 1`
	rpcport=`expr $wsport + 10`
	if [ "$num" == "0" ]; then
		sh -c "$binary $offchain_flags -d db$num --$name --port $newport --ws-port $wsport --rpc-port $rpcport --chain $chain 2>&1" | local_id | logger_for_first_node $tmpdir/port_${newport}_name_$name.txt &
	else
		sh -c "$binary $offchain_flags -d db$num --$name --port $newport --ws-port $wsport --rpc-port $rpcport --chain $chain --bootnodes /ip4/127.0.0.1/tcp/$port/p2p/`cat $localid` 2>&1" | local_id > $tmpdir/port_${newport}_name_$name.txt &
	fi
	echo SCRIPT: "Port:" $newport "P2P port:" $port "Name:" $name "WS:" $wsport "RPC:" $rpcport $tmpdir/port_${newport}_name_$name.txt
	sleep 5
	port="$newport"
	wsport=`expr $wsport + 1`
	num=$(($num + 1))
done

wait

echo SCRIPT: you can stop script by control-C hot key
echo SCRIPT: maybe framenode processes is still runnning, you can check it and finish it by hand
echo SCRIPT: in future this can be done automatically

sleep 999999
