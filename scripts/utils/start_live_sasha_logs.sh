#!/usr/bin/env bash
set -euo pipefail

if ! systemctl --user is-active --quiet sasha.service; then
  echo "Sasha serivce is currently not running..."
  exit 1
fi

journalctl --user -fu sasha.service -f

