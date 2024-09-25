echo "name, cpu, memory" | tee stats.txt


# Gather data loop
while true
do 
    docker stats --format "{{.Name}}, {{.CPUPerc}}, {{.MemUsage}}" --no-stream | tee -a stats.txt
    sleep 0.5s
done