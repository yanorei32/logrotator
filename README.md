# logrotater

A simple log rotater

```bash
mkdir logs

while : ; do
    date
    sleep 1
done | ./logrotater --dir ./ --interval 5

# it will output <UNIXTIME_IN_MS>.log 
```
