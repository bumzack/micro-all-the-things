#!/bin/bash

for s in ls /lib/systemd/system/rust* ; do
	f="$(basename -- $s)"
	echo $f
	sudo systemctl restart $f
done
