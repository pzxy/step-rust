#!/bin/bash

if [ -z "$UDP_LISTENER_NUM_WORKERS" ]
then
	UDP_LISTENER_NUM_WORKERS=20
fi

UDP_LISTENER_KILL_SIGNAL=50
UDP_LISTENER_DISPATCHED=

function terminate_listener
{
	#echo "TERMINATING LISTENERS: $UDP_LISTENER_DISPATCHED"
	kill -$UDP_LISTENER_KILL_SIGNAL $UDP_LISTENER_DISPATCHED

	for i in $UDP_LISTENER_DISPATCHED
	do
		ps -ef | grep nc | grep -v grep | awk '{print $3 " " $2}' | egrep "^$i" | awk '{print $2}' | xargs kill
	done
}

function get_packet
{
	# stop the nc on exit
	trap return $UDP_LISTENER_KILL_SIGNAL

	while [ true ]
	do
		local port=$1

		# note: this combines the packet into one line. We could also just have it print a header and
		# then print a packet and then print a footer, but then we have to figure out some way
		# to mutex the packet printing...
		local packet=$(nc -n -u -l $port -w0)
		echo $packet
	done
}

function run_listener
{
	local port=$1

	trap terminate_listener EXIT

	for i in $(seq 1 $UDP_LISTENER_NUM_WORKERS)
	do
		get_packet $port &
		UDP_LISTENER_DISPATCHED="$UDP_LISTENER_DISPATCHED $!"
	done

	while :
	do
		sleep 1
	done
}

if [ -n "$1" ]
then
	run_listener $1
fi
