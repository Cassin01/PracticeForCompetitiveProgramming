#!bin/zsh
docker cp graph1.png cassinUbuntu01:/root/MyProject/Infexp2/th1
docker cp graph2.png cassinUbuntu01:/root/MyProject/Infexp2/th1
docker cp graph3.png cassinUbuntu01:/root/MyProject/Infexp2/th1

docker cp 01.tex cassinUbuntu01:/root/MyProject/Infexp2/th1
docker exec -it cassinUbuntu01 bash -c "cd /root/MyProject/Infexp2/th1 && platex 01.tex"
docker exec -it cassinUbuntu01 bash -c "cd /root/MyProject/Infexp2/th1 && dvipdfmx 01"
docker cp cassinUbuntu01:/root/MyProject/Infexp2/th1/01.pdf .
