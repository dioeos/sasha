#!/usr/bin/env bash

set -e

mkdir -p ~/.config/systemd/user/

cp /home/dio/tools/sasha/systemd/sasha.service \
   ~/.config/systemd/user/

systemctl --user daemon-reload
systemctl --user enable --now sasha.service
