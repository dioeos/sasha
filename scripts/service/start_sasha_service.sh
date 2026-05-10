#!/usr/bin/env bash
set -euo pipefail
systemctl --user start sasha.service

if ! systemctl --user is-active --quiet sasha.service; then
  echo "Failed to start sasha service:"
  journalctl --user -u sasha.service -n 10
  exit 1
fi

echo "Successfully started sasha service..."
read -p "View live logs: [y/N]" view_logs

if [[ ! "$view_logs" =~ ^[Yy]$ ]]; then
  exit 0
fi

journalctl --user -fu sasha.service
