#!/usr/bin/env bash

set -e

systemctl --user import-environment \
  NIRI_SOCKET \
  WAYLAND_DISPLAY \
  DISPLAY \
  XDG_CURRENT_DESKTOP \
  XDG_SESSION_TYPE \
  XDG_RUNTIME_DIR

mkdir -p ~/.config/systemd/user/

cp /home/dio/tools/sasha/systemd/sasha.service \
   ~/.config/systemd/user/

systemctl --user daemon-reload
systemctl --user enable --now sasha.service
