#!/bin/sh
for FILE in *-in.txt

do
	echo $FILE
	base=${FILE%-in.txt}
    ./matching $FILE > $base.yourname.out.txt # replace with your command!
    diff $base.yourname.out.txt $base-out.txt
done
