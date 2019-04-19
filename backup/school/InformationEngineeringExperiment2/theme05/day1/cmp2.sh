#!bin/zsh
docker cp 01.tex Cassinubuntu01:/root/MyProject/Infexp2/th05
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th05 && platex 01.tex"
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th05 && dvipdfmx 01"
docker cp Cassinubuntu01:/root/MyProject/Infexp2/th05/01.pdf .
