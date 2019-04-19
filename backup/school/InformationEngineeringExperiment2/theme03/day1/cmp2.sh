#!bin/zsh
docker cp MT.cpp cassinUbuntu01:/root/MyProject/Infexp2/th2
docker cp rand.c cassinUbuntu01:/root/MyProject/Infexp2/th2
docker cp graph1.png cassinUbuntu01:/root/MyProject/Infexp2/th2
docker cp 01.tex cassinUbuntu01:/root/MyProject/Infexp2/th2
docker exec -it cassinUbuntu01 bash -c "cd /root/MyProject/Infexp2/th2 && platex 01.tex"
docker exec -it cassinUbuntu01 bash -c "cd /root/MyProject/Infexp2/th2 && dvipdfmx 01"
docker cp cassinUbuntu01:/root/MyProject/Infexp2/th2/01.pdf .
