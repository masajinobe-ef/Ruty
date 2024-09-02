#!/bin/bash

echo "Сборка..."
cargo psp

echo "Подключение к FTP-серверу 192.168.0.133..."
ftp -inv 192.168.0.133 <<EOF
anonymous
delete PSP/GAME/Ruty/EBOOT.PBP
put target/mipsel-sony-psp/debug/EBOOT.PBP PSP/GAME/Ruty/EBOOT.PBP
bye
EOF

echo "Загрузка завершена."
