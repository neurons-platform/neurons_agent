#!/bin/bash
ps -ef |grep neurons_agent |grep -v grep |awk '{print $2}' |xargs -i kill -9 {}
sleep 3
