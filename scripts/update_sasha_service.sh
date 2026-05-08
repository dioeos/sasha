#!/usr/bin/env bash
set -euo pipefail

systemctl --user import-environment \
  NIRI_SOCKET \
  WAYLAND_DISPLAY \
  DISPLAY \
  XDG_CURRENT_DESKTOP \
  XDG_SESSION_TYPE \
  XDG_RUNTIME_DIR

cp /home/dio/tools/sasha/systemd/sasha.service \
   ~/.config/systemd/user/

systemctl --user daemon-reload
systemctl --user restart sasha.service
