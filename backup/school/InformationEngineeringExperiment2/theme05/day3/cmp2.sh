#!bin/zsh
docker cp 01.tex Cassinubuntu01:/root/MyProject/Infexp2/th05/day3
docker cp key2.png Cassinubuntu01:/root/MyProject/Infexp2/th05/day3
docker cp ln_yuudo.png Cassinubuntu01:/root/MyProject/Infexp2/th05/day3
docker cp all_yuudo.png Cassinubuntu01:/root/MyProject/Infexp2/th05/day3

docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th05/day3 && platex 01.tex"
docker exec -it Cassinubuntu01 bash -c "cd /root/MyProject/Infexp2/th05/day3 && dvipdfmx 01"
docker cp Cassinubuntu01:/root/MyProject/Infexp2/th05/day3/01.pdf .
open -a Preview 01.pdf
