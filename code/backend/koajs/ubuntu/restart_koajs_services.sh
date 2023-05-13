#!/bin/bash

for s in ls /lib/systemd/system/koajs* ; do
	f="$(basename -- $s)"
	echo $f
	sudo systemctl restart $f
done
