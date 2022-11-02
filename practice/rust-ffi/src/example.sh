#!/bin/bash
echo "start the udp server listener..."
# the number of listeners defaults to 20 if not specified
UDP_LISTENER_NUM_WORKERS=1 ./udp_listener.sh 3001 2> /dev/null | while read packet
do
	echo "RECEIVED PACKET: $packet"
done
