#!bin/zsh
docker cp 01.tex cassinUbuntu01:/root/MyProject/Infexp2/th1
docker exec -it cassinUbuntu01 platex /root/MyProject/Infexp2/th1/01.tex
docker exec -it cassinUbuntu01 dvipdfmx /01
docker cp cassinUbuntu01:/01.pdf .
