#!/bin/bash

cd /home/ubuntu/boombot/web-frontend

pm2 delete boombot-frontend 2>/dev/null || true

pm2 serve . 3000 --name boombot-frontend --spa

pm2 save

pm2 startup ubuntu || true

pm2 list 