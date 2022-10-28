#!/bin/bash

# the number of listeners defaults to 20 if not specified
UDP_LISTENER_NUM_WORKERS=1 ./udp_listener.sh 3000 2> /dev/null | while read packet
do
	echo "RECEIVED PACKET: $packet"
done
