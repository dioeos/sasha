#!/usr/bin/env bash

cp /home/dio/tools/sasha/systemd/sasha.service \
   ~/.config/systemd/user/

systemctl --user daemon-reload
systemctl --user restart sasha.service
