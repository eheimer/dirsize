#!/bin/bash

function domath {
  echo "scale=1;$1" |bc
}

dirpath=$1
if [ "$dirpath" == "" ]
  then
    dirpath="."
fi
echo Checking $dirpath/
inttotal=0
i=0
prefix=("KB" "MB" "GB")

#parse the du listing and calculate the total
while read size path
do
  rawlisting[$i]="$size $path"
  ((inttotal=$inttotal + $size))
  ((i++))
done < <(du -sk ${dirpath}/.[!.]* ${dirpath}/* 2>/dev/null | sort -n)

#output the lines
for i in "${rawlisting[@]}"
do
  while read intsize path
  do
    percent=$(domath "$intsize*100/$inttotal")
    ((intpercent=$intsize*100/$inttotal))
    if [ "$percent" == "${intpercent}.0" ]; then
      percent=$intpercent
    fi
    size=$intsize
    j=0
    while [ $intsize -ge 1024 ]
    do
      ((intsize=$intsize/1024))
      size=$(domath "$size / 1024")
      ((j++))
    done
    if [ "$size" == "${intsize}.0" -o $intsize -ge 1000 ]; then
      size=$intsize
    fi
    printf "%s%s\t%s%%\t%s\n" "$size" "${prefix[$j]}" "$percent" "$path"
  done < <(echo $i)
done
j=0
total=$inttotal
while [ $inttotal -ge 1024 ]
do
  ((inttotal=$inttotal/1024))
  total=$(domath "$total / 1024")
  ((j++))
done
if [ "$total" == "${inttotal}.0" ]; then
  total=$inttotal
fi
printf "Total: %s%s\n" $total ${prefix[$j]}
