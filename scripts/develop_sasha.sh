#!/usr/bin/env bash

cargo build &&
systemctl --user restart sasha.service &&
journalctl --user -u sasha.service -n 20 --no-pager
