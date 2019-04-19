#!bin/zsh
docker cp key1.png Cassinubuntu01:/root/MyProject/Infexp2/th05/day2
docker cp key2.png Cassinubuntu01:/root/MyProject/Infexp2/th05/day2

docker cp 01.tex Cassinubuntu01:/root/MyProject/Infexp2/th05/day2
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th05/day2 && platex 01.tex"
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th05/day2 && dvipdfmx 01"
docker cp Cassinubuntu01:/root/MyProject/Infexp2/th05/day2/01.pdf .
open -a Preview 01.pdf
