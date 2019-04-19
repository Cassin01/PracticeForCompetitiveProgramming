#!bin/zsh
docker cp Source.cpp Cassinubuntu01:/root/MyProject/Infexp2/th3
docker cp graph1.png Cassinubuntu01:/root/MyProject/Infexp2/th3
docker cp graph2.png Cassinubuntu01:/root/MyProject/Infexp2/th3

docker cp 3-3.tex Cassinubuntu01:/root/MyProject/Infexp2/th3
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th3 && platex 3-3.tex"
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th3 && dvipdfmx 3-3"
docker cp Cassinubuntu01:/root/MyProject/Infexp2/th3/3-3.pdf .
