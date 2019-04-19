#!bin/zsh
docker cp result.png cassinUbuntu01:/root/MyProject/Infexp2/th3
docker cp humming.c cassinUbuntu01:/root/MyProject/Infexp2/th3

docker cp 3-2.tex cassinUbuntu01:/root/MyProject/Infexp2/th3
docker exec -it cassinUbuntu01 bash -c "cd /root/MyProject/Infexp2/th3 && platex 3-2.tex"
docker exec -it cassinUbuntu01 bash -c "cd /root/MyProject/Infexp2/th3 && dvipdfmx 3-2"
docker cp cassinUbuntu01:/root/MyProject/Infexp2/th3/3-2.pdf .
